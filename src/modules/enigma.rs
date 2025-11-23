use crate::module::Module;
use eframe::egui;

/// Historical Enigma rotor wirings (I-VIII)
const ROTOR_WIRINGS: [&str; 8] = [
    "EKMFLGDQVZNTOWYHXUSPAIBRCJ", // Rotor I
    "AJDKSIRUXBLHWTMCQGZNPYFVOE", // Rotor II
    "BDFHJLCPRTXVZNYEIWGAKMUSQO", // Rotor III
    "ESOVPZJAYQUIRHXLNFTGKDCMWB", // Rotor IV
    "VZBRGITYUPSDNHLXAWMJQOFECK", // Rotor V
    "JPGVOUMFYQBENHZRDKASXLICTW", // Rotor VI
    "NZJHGRCXMYSWBOUFAIVLPEKQDT", // Rotor VII
    "FKQHTLXOCBJSPDZRAMEWNIUYGV", // Rotor VIII
];

/// Rotor notch positions (where the next rotor steps)
const ROTOR_NOTCHES: [&str; 8] = [
    "Q",  // Rotor I
    "E",  // Rotor II
    "V",  // Rotor III
    "J",  // Rotor IV
    "Z",  // Rotor V
    "ZM", // Rotor VI (two notches)
    "ZM", // Rotor VII (two notches)
    "ZM", // Rotor VIII (two notches)
];

/// Historical reflector wirings
const REFLECTOR_WIRINGS: [&str; 3] = [
    "YRUHQSLDPXNGOKMIEBFZCWVJAT", // Reflector B
    "FVPJIAOYEDRZXWGCTKUQSBNMHL", // Reflector C
    "ENKQAUYWJICOPBLMDXZVFTHRGS", // Reflector B-Thin
];

#[derive(Clone)]
struct Rotor {
    wiring: String,
    notch: String,
    position: u8,     // 0-25
    ring_setting: u8, // 0-25
}

impl Rotor {
    fn new(rotor_num: usize, position: u8, ring_setting: u8) -> Self {
        Self {
            wiring: ROTOR_WIRINGS[rotor_num].to_string(),
            notch: ROTOR_NOTCHES[rotor_num].to_string(),
            position: position % 26,
            ring_setting: ring_setting % 26,
        }
    }

    fn at_notch(&self) -> bool {
        let pos_char = (b'A' + self.position) as char;
        self.notch.contains(pos_char)
    }

    fn step(&mut self) {
        self.position = (self.position + 1) % 26;
    }

    fn forward(&self, c: u8) -> u8 {
        // Input: 0-25
        let shift = (self.position + 26 - self.ring_setting) % 26;
        let index = (c + shift) % 26;
        let wired = self.wiring.as_bytes()[index as usize] - b'A';
        (wired + 26 - shift) % 26
    }

    fn backward(&self, c: u8) -> u8 {
        // Input: 0-25
        let shift = (self.position + 26 - self.ring_setting) % 26;
        let shifted = (c + shift) % 26;

        // Find the position in wiring
        let wiring_bytes = self.wiring.as_bytes();
        let target = (b'A' + shifted) as char;
        let index = wiring_bytes
            .iter()
            .position(|&b| b as char == target)
            .unwrap_or(0) as u8;

        (index + 26 - shift) % 26
    }
}

struct Reflector {
    wiring: String,
}

impl Reflector {
    fn new(reflector_num: usize) -> Self {
        Self {
            wiring: REFLECTOR_WIRINGS[reflector_num].to_string(),
        }
    }

    fn reflect(&self, c: u8) -> u8 {
        self.wiring.as_bytes()[c as usize] - b'A'
    }
}

struct Plugboard {
    mapping: [u8; 26],
}

impl Plugboard {
    fn new(pairs: &str) -> Self {
        let mut mapping = [0u8; 26];
        for i in 0..26 {
            mapping[i] = i as u8;
        }

        // Parse pairs like "AB CD EF"
        for pair in pairs.split_whitespace() {
            let chars: Vec<char> = pair.chars().collect();
            if chars.len() == 2 {
                let a = (chars[0].to_ascii_uppercase() as u8).saturating_sub(b'A');
                let b = (chars[1].to_ascii_uppercase() as u8).saturating_sub(b'A');
                if a < 26 && b < 26 {
                    mapping[a as usize] = b;
                    mapping[b as usize] = a;
                }
            }
        }

        Self { mapping }
    }

    fn swap(&self, c: u8) -> u8 {
        self.mapping[c as usize]
    }
}

pub struct EnigmaModule {
    // Rotor selection (0-7 for rotors I-VIII)
    left_rotor: usize,
    middle_rotor: usize,
    right_rotor: usize,

    // Rotor positions (A-Z, displayed as 0-25)
    left_position: u8,
    middle_position: u8,
    right_position: u8,

    // Ring settings (A-Z, displayed as 0-25)
    left_ring: u8,
    middle_ring: u8,
    right_ring: u8,

    // Reflector selection (0-2)
    reflector: usize,

    // Plugboard settings
    plugboard_pairs: String,
}

impl Default for EnigmaModule {
    fn default() -> Self {
        Self {
            left_rotor: 0,      // Rotor I
            middle_rotor: 1,    // Rotor II
            right_rotor: 2,     // Rotor III
            left_position: 0,   // A
            middle_position: 0, // A
            right_position: 0,  // A
            left_ring: 0,       // A
            middle_ring: 0,     // A
            right_ring: 0,      // A
            reflector: 0,       // Reflector B
            plugboard_pairs: String::new(),
        }
    }
}

impl EnigmaModule {
    fn encode_char(
        &self,
        c: char,
        rotors: &mut [Rotor; 3],
        reflector: &Reflector,
        plugboard: &Plugboard,
    ) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }

        // Step rotors (double-stepping mechanism)
        let middle_at_notch = rotors[1].at_notch();
        let right_at_notch = rotors[2].at_notch();

        if middle_at_notch {
            rotors[1].step();
            rotors[0].step();
        } else if right_at_notch {
            rotors[1].step();
        }
        rotors[2].step();

        // Convert to 0-25
        let mut signal = c.to_ascii_uppercase() as u8 - b'A';

        // Through plugboard
        signal = plugboard.swap(signal);

        // Through rotors (right to left)
        signal = rotors[2].forward(signal);
        signal = rotors[1].forward(signal);
        signal = rotors[0].forward(signal);

        // Through reflector
        signal = reflector.reflect(signal);

        // Back through rotors (left to right)
        signal = rotors[0].backward(signal);
        signal = rotors[1].backward(signal);
        signal = rotors[2].backward(signal);

        // Through plugboard again
        signal = plugboard.swap(signal);

        // Convert back to char
        (b'A' + signal) as char
    }
}

impl Module for EnigmaModule {
    fn name(&self) -> &str {
        "Enigma Machine"
    }

    fn process(&self, input: &str) -> String {
        // Create rotors with current settings
        let mut rotors = [
            Rotor::new(self.left_rotor, self.left_position, self.left_ring),
            Rotor::new(self.middle_rotor, self.middle_position, self.middle_ring),
            Rotor::new(self.right_rotor, self.right_position, self.right_ring),
        ];

        let reflector = Reflector::new(self.reflector);
        let plugboard = Plugboard::new(&self.plugboard_pairs);

        input
            .chars()
            .map(|c| self.encode_char(c, &mut rotors, &reflector, &plugboard))
            .collect()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Rotor Selection");

        ui.horizontal(|ui| {
            ui.label("Left Rotor:");
            egui::ComboBox::new("left_rotor", "")
                .selected_text(format!("Rotor {}", self.left_rotor + 1))
                .show_ui(ui, |ui| {
                    for i in 0..8 {
                        ui.selectable_value(&mut self.left_rotor, i, format!("Rotor {}", i + 1));
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("Middle Rotor:");
            egui::ComboBox::new("middle_rotor", "")
                .selected_text(format!("Rotor {}", self.middle_rotor + 1))
                .show_ui(ui, |ui| {
                    for i in 0..8 {
                        ui.selectable_value(&mut self.middle_rotor, i, format!("Rotor {}", i + 1));
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("Right Rotor:");
            egui::ComboBox::new("right_rotor", "")
                .selected_text(format!("Rotor {}", self.right_rotor + 1))
                .show_ui(ui, |ui| {
                    for i in 0..8 {
                        ui.selectable_value(&mut self.right_rotor, i, format!("Rotor {}", i + 1));
                    }
                });
        });

        ui.separator();
        ui.heading("Rotor Positions");

        ui.horizontal(|ui| {
            ui.label("Left:");
            let left_char = (b'A' + self.left_position) as char;
            ui.add(
                egui::Slider::new(&mut self.left_position, 0..=25).text(format!("{}", left_char)),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Middle:");
            let middle_char = (b'A' + self.middle_position) as char;
            ui.add(
                egui::Slider::new(&mut self.middle_position, 0..=25)
                    .text(format!("{}", middle_char)),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Right:");
            let right_char = (b'A' + self.right_position) as char;
            ui.add(
                egui::Slider::new(&mut self.right_position, 0..=25).text(format!("{}", right_char)),
            );
        });

        ui.separator();
        ui.heading("Ring Settings");

        ui.horizontal(|ui| {
            ui.label("Left:");
            let left_ring_char = (b'A' + self.left_ring) as char;
            ui.add(
                egui::Slider::new(&mut self.left_ring, 0..=25).text(format!("{}", left_ring_char)),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Middle:");
            let middle_ring_char = (b'A' + self.middle_ring) as char;
            ui.add(
                egui::Slider::new(&mut self.middle_ring, 0..=25)
                    .text(format!("{}", middle_ring_char)),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Right:");
            let right_ring_char = (b'A' + self.right_ring) as char;
            ui.add(
                egui::Slider::new(&mut self.right_ring, 0..=25)
                    .text(format!("{}", right_ring_char)),
            );
        });

        ui.separator();
        ui.heading("Reflector");

        ui.horizontal(|ui| {
            egui::ComboBox::new("reflector", "")
                .selected_text(match self.reflector {
                    0 => "Reflector B",
                    1 => "Reflector C",
                    2 => "Reflector B-Thin",
                    _ => "Unknown",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.reflector, 0, "Reflector B");
                    ui.selectable_value(&mut self.reflector, 1, "Reflector C");
                    ui.selectable_value(&mut self.reflector, 2, "Reflector B-Thin");
                });
        });

        ui.separator();
        ui.heading("Plugboard");
        ui.label("Enter pairs separated by spaces (e.g., 'AB CD EF'):");
        ui.text_edit_singleline(&mut self.plugboard_pairs);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
