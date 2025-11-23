use crate::module::Module;
use eframe::egui;

#[derive(PartialEq, Clone, Copy)]
pub enum CipherMode {
    Encode,
    Decode,
}

pub struct CaesarCipherModule {
    shift: i32,
    mode: CipherMode,
}

impl Default for CaesarCipherModule {
    fn default() -> Self {
        Self {
            shift: 1,
            mode: CipherMode::Encode,
        }
    }
}

impl Module for CaesarCipherModule {
    fn name(&self) -> &str {
        "Caesar Cipher"
    }

    fn process(&self, input: &str) -> String {
        let shift = match self.mode {
            CipherMode::Encode => self.shift.rem_euclid(26) as u8,
            CipherMode::Decode => (26 - self.shift.rem_euclid(26)) as u8,
        };
        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = c as u8 - base;
                    let new_offset = (offset + shift) % 26;
                    (base + new_offset) as char
                } else {
                    c
                }
            })
            .collect()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, CipherMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, CipherMode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("Shift:");
            ui.add(egui::DragValue::new(&mut self.shift));
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
pub struct ROT13Module;

impl Module for ROT13Module {
    fn name(&self) -> &str {
        "ROT13"
    }

    fn process(&self, input: &str) -> String {
        // ROT13 is just Caesar with shift 13
        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = c as u8 - base;
                    let new_offset = (offset + 13) % 26;
                    (base + new_offset) as char
                } else {
                    c
                }
            })
            .collect()
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
pub enum A1Z26Mode {
    Encode,
    Decode,
}

pub struct A1Z26Module {
    mode: A1Z26Mode,
}

impl Default for A1Z26Module {
    fn default() -> Self {
        Self {
            mode: A1Z26Mode::Encode,
        }
    }
}

impl Module for A1Z26Module {
    fn name(&self) -> &str {
        "A1Z26"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            A1Z26Mode::Encode => input
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_alphabetic() {
                        let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                        Some(format!("{}", c as u8 - base + 1))
                    } else if c.is_whitespace() {
                        Some(" ".to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("-"),
            A1Z26Mode::Decode => {
                // Split by non-digit characters
                input
                    .split(|c: char| !c.is_ascii_digit())
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        if let Ok(n) = s.parse::<u8>() {
                            if (1..=26).contains(&n) {
                                (b'a' + n - 1) as char
                            } else {
                                '?'
                            }
                        } else {
                            '?'
                        }
                    })
                    .collect()
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, A1Z26Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, A1Z26Mode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct AffineCipherModule {
    a: i32,
    b: i32,
    mode: CipherMode,
}

impl Default for AffineCipherModule {
    fn default() -> Self {
        Self {
            a: 5,
            b: 8,
            mode: CipherMode::Encode,
        }
    }
}

impl AffineCipherModule {
    /// Calculate modular multiplicative inverse using Extended Euclidean Algorithm
    fn mod_inverse(a: i32, m: i32) -> Option<i32> {
        let (mut t, mut new_t) = (0, 1);
        let (mut r, mut new_r) = (m, a);

        while new_r != 0 {
            let quotient = r / new_r;
            (t, new_t) = (new_t, t - quotient * new_t);
            (r, new_r) = (new_r, r - quotient * new_r);
        }

        if r > 1 {
            return None; // a is not invertible
        }
        if t < 0 {
            t += m;
        }
        Some(t)
    }
}

impl Module for AffineCipherModule {
    fn name(&self) -> &str {
        "Affine Cipher"
    }

    fn process(&self, input: &str) -> String {
        let a = self.a.rem_euclid(26);
        let b = self.b.rem_euclid(26);

        if a % 2 == 0 || a == 13 {
            return format!("Error: 'a' ({}) must be coprime to 26.", a);
        }

        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let x = (c as u8 - base) as i32;
                    let new_x = match self.mode {
                        CipherMode::Encode => (a * x + b).rem_euclid(26),
                        CipherMode::Decode => {
                            // D(y) = a^(-1) * (y - b) mod 26
                            let a_inv = Self::mod_inverse(a, 26).unwrap_or(1);
                            (a_inv * (x - b)).rem_euclid(26)
                        }
                    } as u8;
                    (base + new_x) as char
                } else {
                    c
                }
            })
            .collect()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, CipherMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, CipherMode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("a (Slope):");
            ui.add(egui::DragValue::new(&mut self.a));
            ui.label("b (Intercept):");
            ui.add(egui::DragValue::new(&mut self.b));
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct VigenereCipherModule {
    key: String,
    mode: A1Z26Mode,
}

impl Default for VigenereCipherModule {
    fn default() -> Self {
        Self {
            key: String::from("KEY"),
            mode: A1Z26Mode::Encode,
        }
    }
}

impl Module for VigenereCipherModule {
    fn name(&self) -> &str {
        "Vigenere Cipher"
    }

    fn process(&self, input: &str) -> String {
        let key_clean: Vec<u8> = self
            .key
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase() as u8 - b'A')
            .collect();

        if key_clean.is_empty() {
            return input.to_string();
        }

        let mut key_idx = 0;
        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let x = c as u8 - base;
                    let k = key_clean[key_idx % key_clean.len()];
                    key_idx += 1;

                    let new_x = match self.mode {
                        A1Z26Mode::Encode => (x + k) % 26,
                        A1Z26Mode::Decode => (x + 26 - k) % 26,
                    };
                    (base + new_x) as char
                } else {
                    c
                }
            })
            .collect()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, A1Z26Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, A1Z26Mode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("Key:");
            ui.text_edit_singleline(&mut self.key);
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct RailFenceCipherModule {
    rails: i32,
    mode: A1Z26Mode,
}

impl Default for RailFenceCipherModule {
    fn default() -> Self {
        Self {
            rails: 3,
            mode: A1Z26Mode::Encode,
        }
    }
}

impl Module for RailFenceCipherModule {
    fn name(&self) -> &str {
        "Rail Fence Cipher"
    }

    fn process(&self, input: &str) -> String {
        let rails = self.rails.max(2) as usize;
        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        if len == 0 {
            return String::new();
        }

        match self.mode {
            A1Z26Mode::Encode => {
                let mut fence = vec![vec![]; rails];
                let mut rail = 0;
                let mut direction = 1;

                for c in chars {
                    fence[rail].push(c);
                    if rail == 0 {
                        direction = 1;
                    } else if rail == rails - 1 {
                        direction = -1;
                    }

                    if direction == 1 {
                        rail += 1;
                    } else {
                        rail -= 1;
                    }
                }
                fence.into_iter().flatten().collect()
            }
            A1Z26Mode::Decode => {
                let mut fence = vec![vec![0; len]; rails];
                let mut rail = 0;
                let mut direction = 1;

                for i in 0..len {
                    fence[rail][i] = 1;
                    if rail == 0 {
                        direction = 1;
                    } else if rail == rails - 1 {
                        direction = -1;
                    }
                    if direction == 1 {
                        rail += 1;
                    } else {
                        rail -= 1;
                    }
                }

                let mut char_iter = chars.into_iter();
                let mut filled_fence = vec![vec!['\0'; len]; rails];
                for r in 0..rails {
                    for c in 0..len {
                        if fence[r][c] == 1 {
                            if let Some(ch) = char_iter.next() {
                                filled_fence[r][c] = ch;
                            }
                        }
                    }
                }

                let mut result = String::new();
                rail = 0;
                direction = 1;
                for c in 0..len {
                    result.push(filled_fence[rail][c]);
                    if rail == 0 {
                        direction = 1;
                    } else if rail == rails - 1 {
                        direction = -1;
                    }
                    if direction == 1 {
                        rail += 1;
                    } else {
                        rail -= 1;
                    }
                }
                result
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, A1Z26Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, A1Z26Mode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("Rails:");
            ui.add(egui::DragValue::new(&mut self.rails).range(2..=50));
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct BaconCipherModule {
    mode: A1Z26Mode,
}

impl Default for BaconCipherModule {
    fn default() -> Self {
        Self {
            mode: A1Z26Mode::Encode,
        }
    }
}

impl Module for BaconCipherModule {
    fn name(&self) -> &str {
        "Bacon Cipher"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            A1Z26Mode::Encode => input
                .to_uppercase()
                .chars()
                .map(|c| {
                    if c.is_ascii_alphabetic() {
                        let val = c as u8 - b'A';
                        let mut code = String::new();
                        for i in (0..5).rev() {
                            if (val >> i) & 1 == 0 {
                                code.push('a');
                            } else {
                                code.push('b');
                            }
                        }
                        code + " "
                    } else {
                        c.to_string()
                    }
                })
                .collect(),
            A1Z26Mode::Decode => {
                let clean: String = input
                    .chars()
                    .filter(|c| *c == 'a' || *c == 'b' || *c == 'A' || *c == 'B')
                    .collect();
                let clean = clean.to_lowercase();
                clean
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(5)
                    .map(|chunk| {
                        if chunk.len() == 5 {
                            let mut val = 0;
                            for (i, &c) in chunk.iter().enumerate() {
                                if c == 'b' {
                                    val |= 1 << (4 - i);
                                }
                            }
                            if val < 26 {
                                (b'a' + val) as char
                            } else {
                                '?'
                            }
                        } else {
                            ' '
                        }
                    })
                    .collect()
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, A1Z26Mode::Encode, "Encode");
            ui.radio_value(&mut self.mode, A1Z26Mode::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct AlphabeticalSubstitutionModule {
    plaintext: String,
    ciphertext: String,
    mode: CipherMode,
}

impl Default for AlphabeticalSubstitutionModule {
    fn default() -> Self {
        Self {
            plaintext: "abcdefghijklmnopqrstuvwxyz".to_string(),
            ciphertext: "zyxwvutsrqponmlkjihgfedcba".to_string(),
            mode: CipherMode::Encode,
        }
    }
}

impl Module for AlphabeticalSubstitutionModule {
    fn name(&self) -> &str {
        "Alphabetical Substitution"
    }

    fn process(&self, input: &str) -> String {
        let plain_chars: Vec<char> = self.plaintext.chars().collect();
        let cipher_chars: Vec<char> = self.ciphertext.chars().collect();

        if plain_chars.len() != cipher_chars.len() {
            return "Error: Plaintext and Ciphertext alphabets must have the same length."
                .to_string();
        }

        let mut map = std::collections::HashMap::new();
        // In encode mode: plaintext -> ciphertext
        // In decode mode: ciphertext -> plaintext (swap the mapping)
        let (from_chars, to_chars) = match self.mode {
            CipherMode::Encode => (&plain_chars, &cipher_chars),
            CipherMode::Decode => (&cipher_chars, &plain_chars),
        };

        for (i, &f) in from_chars.iter().enumerate() {
            map.insert(f, to_chars[i]);
            map.insert(f.to_ascii_uppercase(), to_chars[i].to_ascii_uppercase());
        }

        input
            .chars()
            .map(|c| map.get(&c).cloned().unwrap_or(c))
            .collect()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, CipherMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, CipherMode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("Plaintext:");
            ui.text_edit_singleline(&mut self.plaintext);
        });
        ui.horizontal(|ui| {
            ui.label("Ciphertext:");
            ui.text_edit_singleline(&mut self.ciphertext);
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
