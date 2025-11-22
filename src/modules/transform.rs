use crate::module::Module;
use eframe::egui;

#[derive(Default)]
pub struct ReverseModule;

impl Module for ReverseModule {
    fn name(&self) -> &str {
        "Reverse"
    }

    fn process(&self, input: &str) -> String {
        input.chars().rev().collect()
    }

    fn ui(&mut self, _ui: &mut egui::Ui) {
        // No config
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum CaseMode {
    LowerCase,
    UpperCase,
    Capitalize,
    Alternating,
}

pub struct CaseTransformModule {
    mode: CaseMode,
}

impl Default for CaseTransformModule {
    fn default() -> Self {
        Self {
            mode: CaseMode::LowerCase,
        }
    }
}

impl Module for CaseTransformModule {
    fn name(&self) -> &str {
        "Case Transform"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            CaseMode::LowerCase => input.to_lowercase(),
            CaseMode::UpperCase => input.to_uppercase(),
            CaseMode::Capitalize => input
                .split_whitespace()
                .map(|word| {
                    let mut c = word.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" "),
            CaseMode::Alternating => input
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i % 2 == 0 {
                        c.to_lowercase().next().unwrap_or(c)
                    } else {
                        c.to_uppercase().next().unwrap_or(c)
                    }
                })
                .collect(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Mode")
            .selected_text(match self.mode {
                CaseMode::LowerCase => "Lower Case",
                CaseMode::UpperCase => "Upper Case",
                CaseMode::Capitalize => "Capitalize",
                CaseMode::Alternating => "Alternating",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.mode, CaseMode::LowerCase, "Lower Case");
                ui.selectable_value(&mut self.mode, CaseMode::UpperCase, "Upper Case");
                ui.selectable_value(&mut self.mode, CaseMode::Capitalize, "Capitalize");
                ui.selectable_value(&mut self.mode, CaseMode::Alternating, "Alternating");
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
pub struct ReplaceModule {
    find: String,
    replace: String,
}

impl Module for ReplaceModule {
    fn name(&self) -> &str {
        "Replace"
    }

    fn process(&self, input: &str) -> String {
        if self.find.is_empty() {
            input.to_string()
        } else {
            input.replace(&self.find, &self.replace)
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Find:");
            ui.text_edit_singleline(&mut self.find);
            ui.label("Replace with:");
            ui.text_edit_singleline(&mut self.replace);
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum NumeralSystem {
    Decimal,
    Binary,
    Octal,
    Hexadecimal,
}

pub struct NumeralSystemModule {
    from: NumeralSystem,
    to: NumeralSystem,
}

impl Default for NumeralSystemModule {
    fn default() -> Self {
        Self {
            from: NumeralSystem::Decimal,
            to: NumeralSystem::Binary,
        }
    }
}

impl Module for NumeralSystemModule {
    fn name(&self) -> &str {
        "Numeral System"
    }

    fn process(&self, input: &str) -> String {
        // Split by whitespace and process each number
        input
            .split_whitespace()
            .map(|s| {
                let val = match self.from {
                    NumeralSystem::Decimal => s.parse::<i64>().ok(),
                    NumeralSystem::Binary => i64::from_str_radix(s, 2).ok(),
                    NumeralSystem::Octal => i64::from_str_radix(s, 8).ok(),
                    NumeralSystem::Hexadecimal => i64::from_str_radix(s, 16).ok(),
                };

                if let Some(v) = val {
                    match self.to {
                        NumeralSystem::Decimal => format!("{}", v),
                        NumeralSystem::Binary => format!("{:b}", v),
                        NumeralSystem::Octal => format!("{:o}", v),
                        NumeralSystem::Hexadecimal => format!("{:x}", v),
                    }
                } else {
                    s.to_string() // Keep original if parse fails
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("From:");
            egui::ComboBox::from_id_salt("from_sys")
                .selected_text(format!("{:?}", self.from))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.from, NumeralSystem::Decimal, "Decimal");
                    ui.selectable_value(&mut self.from, NumeralSystem::Binary, "Binary");
                    ui.selectable_value(&mut self.from, NumeralSystem::Octal, "Octal");
                    ui.selectable_value(&mut self.from, NumeralSystem::Hexadecimal, "Hexadecimal");
                });
            ui.label("To:");
            egui::ComboBox::from_id_salt("to_sys")
                .selected_text(format!("{:?}", self.to))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.to, NumeralSystem::Decimal, "Decimal");
                    ui.selectable_value(&mut self.to, NumeralSystem::Binary, "Binary");
                    ui.selectable_value(&mut self.to, NumeralSystem::Octal, "Octal");
                    ui.selectable_value(&mut self.to, NumeralSystem::Hexadecimal, "Hexadecimal");
                });
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BitwiseOp {
    NOT,
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
}

pub struct BitwiseOperationModule {
    op: BitwiseOp,
    operand: String, // For binary ops
}

impl Default for BitwiseOperationModule {
    fn default() -> Self {
        Self {
            op: BitwiseOp::NOT,
            operand: "0".to_string(),
        }
    }
}

impl Module for BitwiseOperationModule {
    fn name(&self) -> &str {
        "Bitwise Operation"
    }

    fn process(&self, input: &str) -> String {
        // Treat input as bytes
        let operand_val = self.operand.parse::<u8>().unwrap_or(0);

        let result: Vec<u8> = input
            .bytes()
            .map(|b| match self.op {
                BitwiseOp::NOT => !b,
                BitwiseOp::AND => b & operand_val,
                BitwiseOp::OR => b | operand_val,
                BitwiseOp::XOR => b ^ operand_val,
                BitwiseOp::NAND => !(b & operand_val),
                BitwiseOp::NOR => !(b | operand_val),
                BitwiseOp::XNOR => !(b ^ operand_val),
            })
            .collect();

        // Try to convert back to string, or show hex
        String::from_utf8_lossy(&result).to_string()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Operation")
                .selected_text(format!("{:?}", self.op))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.op, BitwiseOp::NOT, "NOT");
                    ui.selectable_value(&mut self.op, BitwiseOp::AND, "AND");
                    ui.selectable_value(&mut self.op, BitwiseOp::OR, "OR");
                    ui.selectable_value(&mut self.op, BitwiseOp::XOR, "XOR");
                    ui.selectable_value(&mut self.op, BitwiseOp::NAND, "NAND");
                    ui.selectable_value(&mut self.op, BitwiseOp::NOR, "NOR");
                    ui.selectable_value(&mut self.op, BitwiseOp::XNOR, "XNOR");
                });

            if self.op != BitwiseOp::NOT {
                ui.label("Operand (0-255):");
                ui.text_edit_singleline(&mut self.operand);
            }
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
