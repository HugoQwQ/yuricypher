use crate::module::Module;
use eframe::egui;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MORSE_CODE: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('A', ".-");
        m.insert('B', "-...");
        m.insert('C', "-.-.");
        m.insert('D', "-..");
        m.insert('E', ".");
        m.insert('F', "..-.");
        m.insert('G', "--.");
        m.insert('H', "....");
        m.insert('I', "..");
        m.insert('J', ".---");
        m.insert('K', "-.-");
        m.insert('L', ".-..");
        m.insert('M', "--");
        m.insert('N', "-.");
        m.insert('O', "---");
        m.insert('P', ".--.");
        m.insert('Q', "--.-");
        m.insert('R', ".-.");
        m.insert('S', "...");
        m.insert('T', "-");
        m.insert('U', "..-");
        m.insert('V', "...-");
        m.insert('W', ".--");
        m.insert('X', "-..-");
        m.insert('Y', "-.--");
        m.insert('Z', "--..");
        m.insert('1', ".----");
        m.insert('2', "..---");
        m.insert('3', "...--");
        m.insert('4', "....-");
        m.insert('5', ".....");
        m.insert('6', "-....");
        m.insert('7', "--...");
        m.insert('8', "---..");
        m.insert('9', "----.");
        m.insert('0', "-----");
        m
    };
    static ref REVERSE_MORSE_CODE: HashMap<&'static str, char> =
        MORSE_CODE.iter().map(|(k, v)| (*v, *k)).collect();
    static ref NATO_ALPHABET: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('A', "Alfa");
        m.insert('B', "Bravo");
        m.insert('C', "Charlie");
        m.insert('D', "Delta");
        m.insert('E', "Echo");
        m.insert('F', "Foxtrot");
        m.insert('G', "Golf");
        m.insert('H', "Hotel");
        m.insert('I', "India");
        m.insert('J', "Juliett");
        m.insert('K', "Kilo");
        m.insert('L', "Lima");
        m.insert('M', "Mike");
        m.insert('N', "November");
        m.insert('O', "Oscar");
        m.insert('P', "Papa");
        m.insert('Q', "Quebec");
        m.insert('R', "Romeo");
        m.insert('S', "Sierra");
        m.insert('T', "Tango");
        m.insert('U', "Uniform");
        m.insert('V', "Victor");
        m.insert('W', "Whiskey");
        m.insert('X', "X-ray");
        m.insert('Y', "Yankee");
        m.insert('Z', "Zulu");
        m
    };
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Encode,
    Decode,
}

pub struct MorseCodeModule {
    direction: Direction,
}

impl Default for MorseCodeModule {
    fn default() -> Self {
        Self {
            direction: Direction::Encode,
        }
    }
}

impl Module for MorseCodeModule {
    fn name(&self) -> &str {
        "Morse Code"
    }

    fn process(&self, input: &str) -> String {
        match self.direction {
            Direction::Encode => input
                .to_uppercase()
                .chars()
                .map(|c| MORSE_CODE.get(&c).cloned().unwrap_or(" "))
                .collect::<Vec<_>>()
                .join(" "),
            Direction::Decode => input
                .split_whitespace()
                .map(|s| REVERSE_MORSE_CODE.get(s).cloned().unwrap_or(' '))
                .collect::<String>(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.direction, Direction::Encode, "Encode");
            ui.radio_value(&mut self.direction, Direction::Decode, "Decode");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct SpellingAlphabetModule;

impl Default for SpellingAlphabetModule {
    fn default() -> Self {
        Self
    }
}

impl Module for SpellingAlphabetModule {
    fn name(&self) -> &str {
        "Spelling Alphabet"
    }

    fn process(&self, input: &str) -> String {
        input
            .to_uppercase()
            .chars()
            .map(|c| NATO_ALPHABET.get(&c).cloned().unwrap_or(" "))
            .collect::<Vec<_>>()
            .join(" ")
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
