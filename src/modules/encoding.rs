use crate::module::Module;
use base64::prelude::*;
use data_encoding::BASE32;
use eframe::egui;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    Encode,
    Decode,
}

pub struct Base64Module {
    mode: Mode,
}

impl Default for Base64Module {
    fn default() -> Self {
        Self { mode: Mode::Encode }
    }
}

impl Module for Base64Module {
    fn name(&self) -> &str {
        "Base64"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            Mode::Encode => BASE64_STANDARD.encode(input),
            Mode::Decode => match BASE64_STANDARD.decode(input.trim()) {
                Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                Err(_) => "Invalid Base64".to_string(),
            },
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, Mode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Base32 Module
pub struct Base32Module {
    mode: Mode,
}

impl Default for Base32Module {
    fn default() -> Self {
        Self { mode: Mode::Encode }
    }
}

impl Module for Base32Module {
    fn name(&self) -> &str {
        "Base32"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            Mode::Encode => BASE32.encode(input.as_bytes()),
            Mode::Decode => match BASE32.decode(input.trim().as_bytes()) {
                Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                Err(_) => "Invalid Base32".to_string(),
            },
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, Mode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Ascii85 Module
pub struct Ascii85Module {
    mode: Mode,
}

impl Default for Ascii85Module {
    fn default() -> Self {
        Self { mode: Mode::Encode }
    }
}

impl Module for Ascii85Module {
    fn name(&self) -> &str {
        "Ascii85"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            Mode::Encode => encode_ascii85(input.as_bytes()),
            Mode::Decode => match decode_ascii85(input.trim()) {
                Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                Err(_) => "Invalid Ascii85".to_string(),
            },
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, Mode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Ascii85 encoding helper functions
fn encode_ascii85(data: &[u8]) -> String {
    let mut result = String::from("<~");
    let mut i = 0;

    while i < data.len() {
        let mut value: u32 = 0;
        let mut count = 0;

        for j in 0..4 {
            value = value << 8;
            if i + j < data.len() {
                value |= data[i + j] as u32;
                count += 1;
            }
        }

        if count == 4 && value == 0 {
            result.push('z');
        } else {
            let mut encoded = [0u8; 5];
            for j in (0..5).rev() {
                encoded[j] = (value % 85) as u8 + 33;
                value /= 85;
            }

            for j in 0..=count {
                result.push(encoded[j] as char);
            }
        }

        i += 4;
    }

    result.push_str("~>");
    result
}

fn decode_ascii85(data: &str) -> Result<Vec<u8>, String> {
    let data = data.trim_start_matches("<~").trim_end_matches("~>");
    let mut result = Vec::new();
    let mut chars = data.chars().filter(|c| !c.is_whitespace()).peekable();

    while chars.peek().is_some() {
        let mut value: u32 = 0;
        let mut count = 0;

        for _ in 0..5 {
            if let Some(c) = chars.next() {
                if c == 'z' {
                    if count == 0 {
                        result.extend_from_slice(&[0, 0, 0, 0]);
                        break;
                    } else {
                        return Err("Invalid z placement".to_string());
                    }
                }

                if c < '!' || c > 'u' {
                    return Err("Invalid character".to_string());
                }

                value = value * 85 + (c as u32 - 33);
                count += 1;
            } else {
                break;
            }
        }

        if count > 0 {
            for _ in count..5 {
                value = value * 85 + 84;
            }

            let bytes = value.to_be_bytes();
            for i in 0..(count - 1) {
                result.push(bytes[i]);
            }
        }
    }

    Ok(result)
}

// Baudot Code Module
pub struct BaudotCodeModule {
    mode: Mode,
}

impl Default for BaudotCodeModule {
    fn default() -> Self {
        Self { mode: Mode::Encode }
    }
}

impl Module for BaudotCodeModule {
    fn name(&self) -> &str {
        "Baudot Code"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            Mode::Encode => encode_baudot(input),
            Mode::Decode => decode_baudot(input),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, Mode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Baudot Code helper functions
fn get_baudot_letters() -> HashMap<char, u8> {
    let mut map = HashMap::new();
    map.insert('A', 0b00011);
    map.insert('B', 0b11001);
    map.insert('C', 0b01110);
    map.insert('D', 0b01001);
    map.insert('E', 0b00001);
    map.insert('F', 0b01101);
    map.insert('G', 0b11010);
    map.insert('H', 0b10100);
    map.insert('I', 0b00110);
    map.insert('J', 0b01011);
    map.insert('K', 0b01111);
    map.insert('L', 0b10010);
    map.insert('M', 0b11100);
    map.insert('N', 0b01100);
    map.insert('O', 0b11000);
    map.insert('P', 0b10110);
    map.insert('Q', 0b10111);
    map.insert('R', 0b01010);
    map.insert('S', 0b00101);
    map.insert('T', 0b10000);
    map.insert('U', 0b00111);
    map.insert('V', 0b11110);
    map.insert('W', 0b10011);
    map.insert('X', 0b11101);
    map.insert('Y', 0b10101);
    map.insert('Z', 0b10001);
    map.insert(' ', 0b00100);
    map.insert('\r', 0b01000);
    map.insert('\n', 0b00010);
    map
}

fn get_baudot_figures() -> HashMap<char, u8> {
    let mut map = HashMap::new();
    map.insert('-', 0b00011);
    map.insert('?', 0b11001);
    map.insert(':', 0b01110);
    map.insert('$', 0b01001);
    map.insert('3', 0b00001);
    map.insert('!', 0b01101);
    map.insert('&', 0b11010);
    map.insert('#', 0b10100);
    map.insert('8', 0b00110);
    map.insert('\'', 0b01011);
    map.insert('(', 0b01111);
    map.insert(')', 0b10010);
    map.insert('.', 0b11100);
    map.insert(',', 0b01100);
    map.insert('9', 0b11000);
    map.insert('0', 0b10110);
    map.insert('1', 0b10111);
    map.insert('4', 0b01010);
    map.insert('/', 0b00101);
    map.insert('5', 0b10000);
    map.insert('7', 0b00111);
    map.insert('=', 0b11110);
    map.insert('2', 0b10011);
    map.insert('+', 0b11101);
    map.insert('6', 0b10101);
    map.insert('"', 0b10001);
    map.insert(' ', 0b00100);
    map.insert('\r', 0b01000);
    map.insert('\n', 0b00010);
    map
}

fn encode_baudot(input: &str) -> String {
    let letters = get_baudot_letters();
    let figures = get_baudot_figures();
    let mut result = String::new();
    let mut in_figures = false;

    for c in input.to_uppercase().chars() {
        if let Some(&code) = letters.get(&c) {
            if in_figures {
                result.push_str(&format!("{:05b} ", 0b11111)); // Letter shift
                in_figures = false;
            }
            result.push_str(&format!("{:05b} ", code));
        } else if let Some(&code) = figures.get(&c) {
            if !in_figures {
                result.push_str(&format!("{:05b} ", 0b11011)); // Figure shift
                in_figures = true;
            }
            result.push_str(&format!("{:05b} ", code));
        }
    }

    result.trim().to_string()
}

fn decode_baudot(input: &str) -> String {
    let mut letters_rev = HashMap::new();
    for (k, v) in get_baudot_letters() {
        letters_rev.insert(v, k);
    }

    let mut figures_rev = HashMap::new();
    for (k, v) in get_baudot_figures() {
        figures_rev.insert(v, k);
    }

    let mut result = String::new();
    let mut in_figures = false;

    for code_str in input.split_whitespace() {
        if let Ok(code) = u8::from_str_radix(code_str, 2) {
            if code == 0b11111 {
                in_figures = false;
            } else if code == 0b11011 {
                in_figures = true;
            } else if in_figures {
                if let Some(&c) = figures_rev.get(&code) {
                    result.push(c);
                }
            } else {
                if let Some(&c) = letters_rev.get(&code) {
                    result.push(c);
                }
            }
        }
    }

    result
}

#[derive(PartialEq, Clone, Copy)]
enum UnicodeMode {
    Encode,
    Decode,
}

pub struct UnicodeCodePointsModule {
    mode: UnicodeMode,
}

impl Default for UnicodeCodePointsModule {
    fn default() -> Self {
        Self {
            mode: UnicodeMode::Encode,
        }
    }
}

impl Module for UnicodeCodePointsModule {
    fn name(&self) -> &str {
        "Unicode Code Points"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            UnicodeMode::Encode => input
                .chars()
                .map(|c| format!("U+{:04X} ", c as u32))
                .collect(),
            UnicodeMode::Decode => {
                let mut result = String::new();
                for part in input.split_whitespace() {
                    let hex_part = part.trim_start_matches("U+").trim_start_matches("u+");
                    if let Ok(code_point) = u32::from_str_radix(hex_part, 16) {
                        if let Some(c) = char::from_u32(code_point) {
                            result.push(c);
                        }
                    }
                }
                result
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, UnicodeMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, UnicodeMode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(PartialEq, Clone, Copy)]
enum UrlMode {
    Encode,
    Decode,
}

pub struct UrlEncodingModule {
    mode: UrlMode,
}

impl Default for UrlEncodingModule {
    fn default() -> Self {
        Self {
            mode: UrlMode::Encode,
        }
    }
}

impl Module for UrlEncodingModule {
    fn name(&self) -> &str {
        "URL Encoding"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            UrlMode::Encode => input
                .chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
                        c.to_string()
                    } else {
                        format!("%{:02X}", c as u8)
                    }
                })
                .collect(),
            UrlMode::Decode => {
                let mut result = String::new();
                let mut chars = input.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '%' {
                        let hex: String = chars.by_ref().take(2).collect();
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            result.push(byte as char);
                        } else {
                            result.push('%');
                            result.push_str(&hex);
                        }
                    } else if c == '+' {
                        result.push(' ');
                    } else {
                        result.push(c);
                    }
                }
                result
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, UrlMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, UrlMode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Punycode Module
pub struct PunycodeModule {
    mode: Mode,
}

impl Default for PunycodeModule {
    fn default() -> Self {
        Self { mode: Mode::Encode }
    }
}

impl Module for PunycodeModule {
    fn name(&self) -> &str {
        "Punycode"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            Mode::Encode => match idna::domain_to_ascii(input) {
                Ok(encoded) => encoded,
                Err(_) => "Invalid domain".to_string(),
            },
            Mode::Decode => match idna::domain_to_unicode(input) {
                (decoded, Ok(())) => decoded,
                (_, Err(_)) => "Invalid punycode".to_string(),
            },
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, Mode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Bootstring Module (simplified implementation)
pub struct BootstringModule {
    mode: Mode,
}

impl Default for BootstringModule {
    fn default() -> Self {
        Self { mode: Mode::Encode }
    }
}

impl Module for BootstringModule {
    fn name(&self) -> &str {
        "Bootstring"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            Mode::Encode => {
                // Simplified bootstring: just show which chars are ASCII vs non-ASCII
                let ascii_part: String = input.chars().filter(|c| c.is_ascii()).collect();
                let non_ascii: Vec<char> = input.chars().filter(|c| !c.is_ascii()).collect();

                if non_ascii.is_empty() {
                    ascii_part
                } else {
                    format!(
                        "{}-{}",
                        ascii_part,
                        non_ascii
                            .iter()
                            .map(|c| format!("{:x}", *c as u32))
                            .collect::<Vec<_>>()
                            .join("-")
                    )
                }
            }
            Mode::Decode => {
                // Simplified decode
                if let Some(dash_pos) = input.rfind('-') {
                    let ascii_part = &input[..dash_pos];
                    let encoded_part = &input[dash_pos + 1..];

                    let mut result = ascii_part.to_string();
                    for hex_str in encoded_part.split('-') {
                        if let Ok(code_point) = u32::from_str_radix(hex_str, 16) {
                            if let Some(c) = char::from_u32(code_point) {
                                result.push(c);
                            }
                        }
                    }
                    result
                } else {
                    input.to_string()
                }
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, Mode::Decode, "Decode");
        });
        ui.label("Note: Simplified Bootstring implementation");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(PartialEq, Clone, Copy)]
enum IntegerMode {
    ToDecimal,
    ToHex,
}

pub struct IntegerModule {
    mode: IntegerMode,
}

impl Default for IntegerModule {
    fn default() -> Self {
        Self {
            mode: IntegerMode::ToDecimal,
        }
    }
}

impl Module for IntegerModule {
    fn name(&self) -> &str {
        "Integer"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            IntegerMode::ToDecimal => input.bytes().map(|b| format!("{} ", b)).collect(),
            IntegerMode::ToHex => input.bytes().map(|b| format!("{:02X} ", b)).collect(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, IntegerMode::ToDecimal, "To Decimal");
            ui.radio_value(&mut self.mode, IntegerMode::ToHex, "To Hex");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
