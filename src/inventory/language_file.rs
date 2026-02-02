use crate::inventory::items_game::GameTranslation;
use std::path::Path;

pub struct LanguageFileParser;

impl LanguageFileParser {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<GameTranslation, LanguageFileLoadError> {
        let bytes = std::fs::read(path)
            .map_err(|e| LanguageFileLoadError::Io(e))?;

        let content = decode_utf16_le(&bytes)
            .map_err(|e| LanguageFileLoadError::Parse(format!("UTF-16 decode error: {}", e)))?;

        Self::parse_from_str(&content)
    }

    pub fn parse_from_str(content: &str) -> Result<GameTranslation, LanguageFileLoadError> {
        let mut translation = GameTranslation::default();

        // Simple line-by-line parser
        let mut in_tokens = false;
        let mut brace_depth = 0;

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            // Check for Tokens section
            if trimmed == "\"Tokens\"" || trimmed.starts_with("\"Tokens\"") {
                in_tokens = true;
                continue;
            }

            // Track braces
            if trimmed == "{" {
                if in_tokens {
                    brace_depth += 1;
                }
                continue;
            }

            if trimmed == "}" {
                if in_tokens && brace_depth > 0 {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        in_tokens = false;
                    }
                }
                continue;
            }

            // Parse key-value pairs
            if in_tokens && brace_depth > 0 {
                if let Some((key, value)) = Self::parse_key_value_line(trimmed) {
                    translation.insert(key, value);
                }
            }
        }

        Ok(translation)
    }

    fn parse_key_value_line(line: &str) -> Option<(String, String)> {
        let mut chars = line.chars().peekable();
        let mut key = String::new();
        let mut value = String::new();

        // Skip leading whitespace
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        // Parse key (must be in quotes)
        if chars.peek() != Some(&'"') {
            return None;
        }
        chars.next(); // consume opening quote

        let mut escaped = false;
        while let Some(ch) = chars.next() {
            if escaped {
                key.push(match ch {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '"' => '"',
                    '\\' => '\\',
                    _ => ch,
                });
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                break;
            } else {
                key.push(ch);
            }
        }

        // Skip whitespace between key and value
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        // Parse value (must be in quotes)
        if chars.peek() != Some(&'"') {
            return None;
        }
        chars.next(); // consume opening quote

        escaped = false;
        while let Some(ch) = chars.next() {
            if escaped {
                value.push(match ch {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '"' => '"',
                    '\\' => '\\',
                    _ => ch,
                });
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                break;
            } else {
                value.push(ch);
            }
        }

        if key.is_empty() {
            None
        } else {
            Some((key, value))
        }
    }
}

fn decode_utf16_le(bytes: &[u8]) -> Result<String, std::string::FromUtf16Error> {
    let u16_slice: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    String::from_utf16(&u16_slice)
}

#[derive(Debug)]
pub enum LanguageFileLoadError {
    Io(std::io::Error),
    Parse(String),
}

impl std::fmt::Display for LanguageFileLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageFileLoadError::Io(e) => write!(f, "IO Error: {}", e),
            LanguageFileLoadError::Parse(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl std::error::Error for LanguageFileLoadError {}
