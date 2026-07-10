use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

const SERVERDATA_RESPONSE_VALUE: i32 = 0;
const SERVERDATA_EXECCOMMAND: i32 = 2;
const SERVERDATA_AUTH_RESPONSE: i32 = 2;
const SERVERDATA_AUTH: i32 = 3;
const MIN_PACKET_SIZE: i32 = 10;
const MAX_PACKET_SIZE: i32 = 4096;

#[derive(Debug)]
struct RconPacket {
    id: i32,
    packet_type: i32,
    body: String,
}

pub struct RconClient {
    stream: TcpStream,
    next_id: i32,
}

impl RconClient {
    pub fn connect(
        address: &str,
        port: u16,
        password: &str,
        timeout: Duration,
    ) -> Result<Self, String> {
        let mut addrs = (address, port)
            .to_socket_addrs()
            .map_err(|e| format!("Failed to resolve address: {}", e))?;

        let socket_addr = addrs
            .next()
            .ok_or_else(|| "No socket address resolved".to_string())?;

        let mut stream =
            TcpStream::connect_timeout(&socket_addr, timeout).map_err(|e| e.to_string())?;
        stream
            .set_read_timeout(Some(timeout))
            .map_err(|e| e.to_string())?;
        stream
            .set_write_timeout(Some(timeout))
            .map_err(|e| e.to_string())?;

        let auth_id = 1;
        write_packet(&mut stream, auth_id, SERVERDATA_AUTH, password)?;

        let mut authenticated = false;
        for _ in 0..2 {
            let packet = read_packet(&mut stream)?;
            if packet.packet_type == SERVERDATA_AUTH_RESPONSE {
                if packet.id == -1 {
                    return Err("RCON authentication failed".to_string());
                }
                if packet.id == auth_id {
                    authenticated = true;
                    break;
                }
            }
        }

        if !authenticated {
            return Err("RCON authentication response not received".to_string());
        }

        Ok(Self { stream, next_id: 2 })
    }

    pub fn send_command(&mut self, command: &str) -> Result<String, String> {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let terminator_id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        write_packet(&mut self.stream, id, SERVERDATA_EXECCOMMAND, command)?;
        write_packet(&mut self.stream, terminator_id, SERVERDATA_EXECCOMMAND, "")?;

        let mut body = String::new();
        loop {
            let packet = read_packet(&mut self.stream)?;
            if packet.id == terminator_id {
                if packet.packet_type != SERVERDATA_RESPONSE_VALUE {
                    return Err(format!(
                        "Unexpected RCON terminator packet type {}",
                        packet.packet_type
                    ));
                }
                break;
            }
            if packet.id != id {
                return Err(format!("Unexpected RCON response id {}", packet.id));
            }
            if packet.packet_type != SERVERDATA_RESPONSE_VALUE {
                return Err(format!(
                    "Unexpected RCON response packet type {}",
                    packet.packet_type
                ));
            }
            body.push_str(&packet.body);
        }
        Ok(body)
    }
}

fn write_packet(
    stream: &mut TcpStream,
    id: i32,
    packet_type: i32,
    body: &str,
) -> Result<(), String> {
    let body_bytes = body.as_bytes();
    let size = 4 + 4 + body_bytes.len() + 2;
    if size > i32::MAX as usize {
        return Err("RCON packet is too large".to_string());
    }

    let mut bytes = Vec::with_capacity(4 + size);
    bytes.extend_from_slice(&(size as i32).to_le_bytes());
    bytes.extend_from_slice(&id.to_le_bytes());
    bytes.extend_from_slice(&packet_type.to_le_bytes());
    bytes.extend_from_slice(body_bytes);
    bytes.push(0);
    bytes.push(0);

    stream.write_all(&bytes).map_err(|e| e.to_string())
}

fn read_packet(stream: &mut TcpStream) -> Result<RconPacket, String> {
    let mut size_bytes = [0u8; 4];
    stream
        .read_exact(&mut size_bytes)
        .map_err(|e| format!("Failed to read RCON packet size: {}", e))?;
    let size = i32::from_le_bytes(size_bytes);
    if !(MIN_PACKET_SIZE..=MAX_PACKET_SIZE).contains(&size) {
        return Err(format!("Invalid RCON packet size {}", size));
    }

    let mut payload = vec![0u8; size as usize];
    stream
        .read_exact(&mut payload)
        .map_err(|e| format!("Failed to read RCON packet payload: {}", e))?;

    let id = i32::from_le_bytes(payload[0..4].try_into().expect("packet id bytes"));
    let packet_type = i32::from_le_bytes(payload[4..8].try_into().expect("packet type bytes"));
    let body_bytes = &payload[8..payload.len().saturating_sub(2)];
    let body = String::from_utf8_lossy(body_bytes).to_string();

    Ok(RconPacket {
        id,
        packet_type,
        body,
    })
}
