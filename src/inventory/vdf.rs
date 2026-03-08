use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum VdfValue {
    String(String),
    Object(HashMap<String, VdfValue>),
}

impl VdfValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            VdfValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, VdfValue>> {
        match self {
            VdfValue::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, VdfValue>> {
        match self {
            VdfValue::Object(o) => Some(o),
            _ => None,
        }
    }
}

pub struct VdfParser;

impl VdfParser {
    pub fn parse(content: &str) -> Result<HashMap<String, VdfValue>, VdfParseError> {
        let mut parser = VdfTokenizer::new(content);
        parser.skip_whitespace();

        if parser.consume("{") {
            return Self::parse_object(&mut parser);
        }

        let mut obj = HashMap::new();

        while parser.position < parser.content.len() {
            let key = parser.parse_key()?;
            parser.skip_whitespace();

            if parser.consume("{") {
                obj.insert(key, VdfValue::Object(Self::parse_object(&mut parser)?));
            } else {
                let value = parser.parse_string()?;
                obj.insert(key, VdfValue::String(value));
            }

            parser.skip_whitespace();
        }

        Ok(obj)
    }

    fn parse_object(
        parser: &mut VdfTokenizer,
    ) -> Result<HashMap<String, VdfValue>, VdfParseError> {
        let mut obj = HashMap::new();

        loop {
            parser.skip_whitespace();

            if parser.peek("}") {
                break;
            }

            let key = parser.parse_key()?;
            parser.skip_whitespace();

            if parser.consume("{") {
                let new_obj = Self::parse_object(parser)?;
                // If key already exists and both are objects, merge them
                if let Some(VdfValue::Object(existing)) = obj.get_mut(&key) {
                    for (k, v) in new_obj {
                        existing.insert(k, v);
                    }
                } else {
                    obj.insert(key, VdfValue::Object(new_obj));
                }
            } else {
                let value = parser.parse_string()?;
                obj.insert(key, VdfValue::String(value));
            }
            
            parser.skip_whitespace();
        }

        parser.expect("}")?;
        Ok(obj)
    }

    pub fn to_string(value: &VdfValue) -> String {
        Self::to_string_internal(value, 0)
    }

    fn to_string_internal(value: &VdfValue, depth: usize) -> String {
        match value {
            VdfValue::String(s) => format!("\"{}\"", Self::escape_string(s)),
            VdfValue::Object(o) => {
                let indent_str = "\t".repeat(depth);
                let mut result = String::new();
                
                let mut keys: Vec<_> = o.keys().collect();
                
                let item_field_order = vec![
                    "inventory", "def_index", "level", "quality", "flags", "origin",
                    "custom_name", "in_use", "rarity", "attributes", "equipped_state"
                ];
                
                let has_item_fields = o.keys().any(|k| item_field_order.contains(&k.as_str()));
                
                let is_attributes_or_equipped = o.keys().all(|k| k.parse::<u64>().is_ok()) 
                    && !has_item_fields;
                
                let is_root_level = depth == 0 && o.contains_key("items") && o.contains_key("default_equips");
                
                let is_item_object = has_item_fields && o.keys().all(|k| {
                    item_field_order.contains(&k.as_str()) || k.parse::<u64>().is_ok()
                });
                
                if is_root_level {
                    keys.sort_by(|a, b| {
                        let a_priority = if a.as_str() == "items" { 0 } else if a.as_str() == "default_equips" { 1 } else { 2 };
                        let b_priority = if b.as_str() == "items" { 0 } else if b.as_str() == "default_equips" { 1 } else { 2 };
                        a_priority.cmp(&b_priority)
                    });
                } else if is_item_object {
                    keys.sort_by(|a, b| {
                        let a_idx = item_field_order.iter().position(|&x| x == a.as_str()).unwrap_or(usize::MAX);
                        let b_idx = item_field_order.iter().position(|&x| x == b.as_str()).unwrap_or(usize::MAX);
                        a_idx.cmp(&b_idx)
                    });
                } else if is_attributes_or_equipped {
                    keys.sort_by(|a, b| {
                        let a_num = a.parse::<u64>().unwrap_or(u64::MAX);
                        let b_num = b.parse::<u64>().unwrap_or(u64::MAX);
                        a_num.cmp(&b_num)
                    });
                } else {
                    keys.sort_by(|a, b| {
                        let a_num = a.parse::<u64>();
                        let b_num = b.parse::<u64>();
                        match (a_num, b_num) {
                            (Ok(a), Ok(b)) => a.cmp(&b),
                            (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                            (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                            (Err(_), Err(_)) => a.cmp(b),
                        }
                    });
                }
                
                for key in keys {
                    let val = o.get(key).unwrap();
                    match val {
                        VdfValue::String(s) => {
                            result.push_str(&format!("{}\"{}\"\t\t\"{}\"\n", indent_str, key, Self::escape_string(s)));
                        }
                        VdfValue::Object(_inner) => {
                            result.push_str(&format!("{}\"{}\"\n{}{{\n", indent_str, key, indent_str));
                            result.push_str(&Self::to_string_internal(val, depth + 1));
                            result.push_str(&format!("{}}}\n", indent_str));
                        }
                    }
                }
                result
            }
        }
    }

    fn escape_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }
}

#[derive(Debug, PartialEq)]
pub struct VdfParseError {
    pub message: String,
    pub position: usize,
}

impl std::fmt::Display for VdfParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VDF Parse Error at position {}: {}", self.position, self.message)
    }
}

impl std::error::Error for VdfParseError {}

struct VdfTokenizer<'a> {
    content: &'a str,
    position: usize,
}

impl<'a> VdfTokenizer<'a> {
    fn new(content: &'a str) -> Self {
        Self {
            content,
            position: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.content.len() {
            match self.content.as_bytes()[self.position] {
                b' ' | b'\n' | b'\r' | b'\t' => self.position += 1,
                _ => break,
            }
        }
    }

    fn peek(&self, s: &str) -> bool {
        self.content[self.position..].starts_with(s)
    }

    fn consume(&mut self, s: &str) -> bool {
        if self.peek(s) {
            self.position += s.len();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, s: &str) -> Result<(), VdfParseError> {
        self.skip_whitespace();
        if self.consume(s) {
            Ok(())
        } else {
            Err(VdfParseError {
                message: format!("Expected '{}'", s),
                position: self.position,
            })
        }
    }

    fn parse_key(&mut self) -> Result<String, VdfParseError> {
        self.skip_whitespace();

        if self.peek("\"") {
            self.parse_string()
        } else {
            self.parse_unquoted_key()
        }
    }

    fn parse_unquoted_key(&mut self) -> Result<String, VdfParseError> {
        let start = self.position;

        while self.position < self.content.len() {
            let ch = self.content.as_bytes()[self.position];
            if ch == b'{' || ch == b'}' || ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r' {
                break;
            }
            self.position += 1;
        }

        let key = &self.content[start..self.position];
        if key.is_empty() {
            return Err(VdfParseError {
                message: "Expected key".to_string(),
                position: self.position,
            });
        }

        Ok(key.to_string())
    }

    fn parse_string(&mut self) -> Result<String, VdfParseError> {
        if !self.consume("\"") {
            return Err(VdfParseError {
                message: "Expected '\"'".to_string(),
                position: self.position,
            });
        }

        let mut result = String::new();
        let mut escaped = false;

        while self.position < self.content.len() {
            let ch = self.content.as_bytes()[self.position];

            if escaped {
                result.push(match ch {
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'"' => '"',
                    b'\\' => '\\',
                    _ => ch as char,
                });
                escaped = false;
                self.position += 1;
                continue;
            }

            match ch {
                b'\\' => {
                    escaped = true;
                    self.position += 1;
                }
                b'"' => {
                    self.position += 1;
                    return Ok(result);
                }
                _ => {
                    result.push(ch as char);
                    self.position += 1;
                }
            }
        }

        Err(VdfParseError {
            message: "Unterminated string".to_string(),
            position: self.position,
        })
    }
}
