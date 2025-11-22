use crate::module::Module;
use base64::prelude::*;
use eframe::egui;

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

// Placeholders for other encoding modules
#[derive(Default)]
pub struct Base32Module;
impl Module for Base32Module {
    fn name(&self) -> &str {
        "Base32"
    }
    fn process(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
    fn ui(&mut self, _ui: &mut egui::Ui) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Default)]
pub struct Ascii85Module;
impl Module for Ascii85Module {
    fn name(&self) -> &str {
        "Ascii85"
    }
    fn process(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
    fn ui(&mut self, _ui: &mut egui::Ui) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Default)]
pub struct BaudotCodeModule;
impl Module for BaudotCodeModule {
    fn name(&self) -> &str {
        "Baudot Code"
    }
    fn process(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
    fn ui(&mut self, _ui: &mut egui::Ui) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
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

#[derive(Default)]
pub struct PunycodeModule;
impl Module for PunycodeModule {
    fn name(&self) -> &str {
        "Punycode"
    }
    fn process(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
    fn ui(&mut self, _ui: &mut egui::Ui) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Default)]
pub struct BootstringModule;
impl Module for BootstringModule {
    fn name(&self) -> &str {
        "Bootstring"
    }
    fn process(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
    fn ui(&mut self, _ui: &mut egui::Ui) {}
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
