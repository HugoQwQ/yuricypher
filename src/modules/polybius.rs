use crate::module::Module;
use eframe::egui;

#[derive(PartialEq, Clone, Copy)]
pub enum PolybiusMode {
    Encode,
    Decode,
}

pub struct PolybiusSquareModule {
    key: String,
    size: usize, // 5 for 5x5, 6 for 6x6
    pub mode: PolybiusMode,
}

impl Default for PolybiusSquareModule {
    fn default() -> Self {
        Self {
            key: String::new(),
            size: 5,
            mode: PolybiusMode::Encode,
        }
    }
}

impl Module for PolybiusSquareModule {
    fn name(&self) -> &str {
        "Polybius Square"
    }

    fn process(&self, input: &str) -> String {
        let square = self.generate_square();

        match self.mode {
            PolybiusMode::Encode => {
                let mut result = String::new();
                for c in input.to_uppercase().chars() {
                    if let Some(pos) = self.find_in_square(&square, c) {
                        let row = pos / self.size;
                        let col = pos % self.size;
                        result.push_str(&format!("{}{}", row + 1, col + 1));
                        result.push(' ');
                    } else {
                        result.push(c);
                    }
                }
                result
            }
            PolybiusMode::Decode => {
                let mut result = String::new();
                let digits: Vec<char> = input.chars().filter(|c| c.is_ascii_digit()).collect();

                for pair in digits.chunks(2) {
                    if pair.len() == 2 {
                        if let (Some(r), Some(c)) = (pair[0].to_digit(10), pair[1].to_digit(10)) {
                            let row = r as usize;
                            let col = c as usize;
                            if row > 0 && col > 0 && row <= self.size && col <= self.size {
                                let pos = (row - 1) * self.size + (col - 1);
                                if pos < square.len() {
                                    result.push(square[pos]);
                                }
                            }
                        }
                    }
                }
                result
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, PolybiusMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, PolybiusMode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("Grid Size:");
            ui.radio_value(&mut self.size, 5, "5×5 (I/J merged)");
            ui.radio_value(&mut self.size, 6, "6×6 (with digits)");
        });

        ui.horizontal(|ui| {
            ui.label("Custom Key:");
            ui.text_edit_singleline(&mut self.key);
            if ui.button("Clear").clicked() {
                self.key.clear();
            }
        });

        ui.label("Leave key empty for standard alphabetical order");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl PolybiusSquareModule {
    /// Generate the Polybius square based on key and size
    fn generate_square(&self) -> Vec<char> {
        let mut square = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Add characters from key first (deduplicated)
        for c in self.key.to_uppercase().chars() {
            if c.is_ascii_alphanumeric() && !seen.contains(&c) {
                let normalized = if self.size == 5 && c == 'J' { 'I' } else { c };
                if !seen.contains(&normalized) {
                    square.push(normalized);
                    seen.insert(normalized);
                }
            }
        }

        // Fill remaining with alphabet (and digits for 6x6)
        if self.size == 5 {
            // 5x5: A-Z with I/J merged (25 cells)
            for c in 'A'..='Z' {
                if c == 'J' {
                    continue;
                } // Skip J, use I instead
                if !seen.contains(&c) {
                    square.push(c);
                    seen.insert(c);
                }
            }
        } else {
            // 6x6: A-Z + 0-9 (36 cells)
            for c in 'A'..='Z' {
                if !seen.contains(&c) {
                    square.push(c);
                    seen.insert(c);
                }
            }
            for c in '0'..='9' {
                if !seen.contains(&c) {
                    square.push(c);
                    seen.insert(c);
                }
            }
        }

        square
    }

    /// Find the position of a character in the square
    fn find_in_square(&self, square: &[char], c: char) -> Option<usize> {
        let search_char = if self.size == 5 && c == 'J' { 'I' } else { c };
        square.iter().position(|&ch| ch == search_char)
    }
}

pub struct ADFGXCipherModule {
    polybius_key: String,
    transposition_key: String,
    mode: PolybiusMode,
}

impl Default for ADFGXCipherModule {
    fn default() -> Self {
        Self {
            polybius_key: String::new(),
            transposition_key: String::new(),
            mode: PolybiusMode::Encode,
        }
    }
}

impl Module for ADFGXCipherModule {
    fn name(&self) -> &str {
        "ADFGX Cipher"
    }

    fn process(&self, input: &str) -> String {
        // 1. Generate 5x5 Polybius Square (I/J merged)
        let mut poly = PolybiusSquareModule::default();
        poly.key = self.polybius_key.clone();
        poly.size = 5;
        let square = poly.generate_square();
        let headers = ['A', 'D', 'F', 'G', 'X'];

        match self.mode {
            PolybiusMode::Encode => {
                // Step 1: Substitution
                let mut substituted = String::new();
                for c in input.to_uppercase().chars() {
                    if let Some(pos) = poly.find_in_square(&square, c) {
                        let row = pos / 5;
                        let col = pos % 5;
                        substituted.push(headers[row]);
                        substituted.push(headers[col]);
                    }
                }

                // Step 2: Columnar Transposition
                let key = self.transposition_key.to_uppercase();
                let key_chars: Vec<char> =
                    key.chars().filter(|c| c.is_ascii_alphabetic()).collect();
                if key_chars.is_empty() {
                    return substituted;
                }

                let num_cols = key_chars.len();
                let num_rows = (substituted.len() + num_cols - 1) / num_cols;
                let mut grid = vec![vec![' '; num_cols]; num_rows];
                let sub_chars: Vec<char> = substituted.chars().collect();

                for (i, &c) in sub_chars.iter().enumerate() {
                    grid[i / num_cols][i % num_cols] = c;
                }

                // Sort key to determine column order
                let mut key_indices: Vec<usize> = (0..num_cols).collect();
                key_indices.sort_by_key(|&i| key_chars[i]);

                let mut result = String::new();
                for &col_idx in &key_indices {
                    for row in 0..num_rows {
                        let c = grid[row][col_idx];
                        if c != ' ' {
                            result.push(c);
                        }
                    }
                    result.push(' '); // Space between columns for readability
                }
                result
            }
            PolybiusMode::Decode => {
                let input_clean: String = input.chars().filter(|c| "ADFGX".contains(*c)).collect();
                let key = self.transposition_key.to_uppercase();
                let key_chars: Vec<char> =
                    key.chars().filter(|c| c.is_ascii_alphabetic()).collect();

                if key_chars.is_empty() || input_clean.is_empty() {
                    return String::new();
                }

                let num_cols = key_chars.len();
                let total_len = input_clean.len();
                let num_rows = (total_len + num_cols - 1) / num_cols;
                let num_full_cols = total_len % num_cols; // Columns that have full rows
                let num_full_cols = if num_full_cols == 0 {
                    num_cols
                } else {
                    num_full_cols
                };

                // Determine column lengths
                let mut col_lengths = vec![num_rows - 1; num_cols];
                for i in 0..num_full_cols {
                    col_lengths[i] = num_rows;
                }

                // Sort key to determine reading order
                let mut key_indices: Vec<usize> = (0..num_cols).collect();
                key_indices.sort_by_key(|&i| key_chars[i]);

                // Fill columns based on sorted key
                let mut grid = vec![vec![' '; num_cols]; num_rows];
                let mut current_idx = 0;
                let input_chars: Vec<char> = input_clean.chars().collect();

                for &col_idx in &key_indices {
                    let len = col_lengths[col_idx];
                    for row in 0..len {
                        if current_idx < input_chars.len() {
                            grid[row][col_idx] = input_chars[current_idx];
                            current_idx += 1;
                        }
                    }
                }

                // Read rows to get substituted text
                let mut substituted = String::new();
                for row in 0..num_rows {
                    for col in 0..num_cols {
                        let c = grid[row][col];
                        if c != ' ' {
                            substituted.push(c);
                        }
                    }
                }

                // Reverse Substitution
                let mut result = String::new();
                let sub_chars: Vec<char> = substituted.chars().collect();
                for pair in sub_chars.chunks(2) {
                    if pair.len() == 2 {
                        let r_char = pair[0];
                        let c_char = pair[1];
                        if let (Some(r), Some(c)) = (
                            headers.iter().position(|&h| h == r_char),
                            headers.iter().position(|&h| h == c_char),
                        ) {
                            let pos = r * 5 + c;
                            if pos < square.len() {
                                result.push(square[pos]);
                            }
                        }
                    }
                }
                result
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, PolybiusMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, PolybiusMode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("Polybius Key:");
            ui.text_edit_singleline(&mut self.polybius_key);
        });
        ui.horizontal(|ui| {
            ui.label("Transposition Key:");
            ui.text_edit_singleline(&mut self.transposition_key);
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct BifidCipherModule {
    key: String,
    mode: PolybiusMode,
}

impl Default for BifidCipherModule {
    fn default() -> Self {
        Self {
            key: String::new(),
            mode: PolybiusMode::Encode,
        }
    }
}

impl Module for BifidCipherModule {
    fn name(&self) -> &str {
        "Bifid Cipher"
    }

    fn process(&self, input: &str) -> String {
        let mut poly = PolybiusSquareModule::default();
        poly.key = self.key.clone();
        poly.size = 5;
        let square = poly.generate_square();

        match self.mode {
            PolybiusMode::Encode => {
                let mut rows = Vec::new();
                let mut cols = Vec::new();

                // 1. Get coordinates
                for c in input.to_uppercase().chars() {
                    if let Some(pos) = poly.find_in_square(&square, c) {
                        rows.push(pos / 5);
                        cols.push(pos % 5);
                    }
                }

                // 2. Combine rows and cols
                let mut combined = rows;
                combined.extend(cols);

                // 3. Read pairs and convert back to letters
                let mut result = String::new();
                for pair in combined.chunks(2) {
                    if pair.len() == 2 {
                        let pos = pair[0] * 5 + pair[1];
                        if pos < square.len() {
                            result.push(square[pos]);
                        }
                    }
                }
                result
            }
            PolybiusMode::Decode => {
                let mut coords = Vec::new();
                for c in input.to_uppercase().chars() {
                    if let Some(pos) = poly.find_in_square(&square, c) {
                        coords.push(pos / 5);
                        coords.push(pos % 5);
                    }
                }

                if coords.len() % 2 != 0 {
                    return "Error: Odd number of coordinates".to_string();
                }

                let mid = coords.len() / 2;
                let rows = &coords[0..mid];
                let cols = &coords[mid..];

                let mut result = String::new();
                for i in 0..mid {
                    let pos = rows[i] * 5 + cols[i];
                    if pos < square.len() {
                        result.push(square[pos]);
                    }
                }
                result
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, PolybiusMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, PolybiusMode::Decode, "Decode");
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

pub struct NihilistCipherModule {
    polybius_key: String,
    keyword: String,
    mode: PolybiusMode,
}

impl Default for NihilistCipherModule {
    fn default() -> Self {
        Self {
            polybius_key: String::new(),
            keyword: String::new(),
            mode: PolybiusMode::Encode,
        }
    }
}

impl Module for NihilistCipherModule {
    fn name(&self) -> &str {
        "Nihilist Cipher"
    }

    fn process(&self, input: &str) -> String {
        let mut poly = PolybiusSquareModule::default();
        poly.key = self.polybius_key.clone();
        poly.size = 5;
        let square = poly.generate_square();

        // Convert keyword to coordinates
        let mut key_coords = Vec::new();
        for c in self.keyword.to_uppercase().chars() {
            if let Some(pos) = poly.find_in_square(&square, c) {
                let row = pos / 5 + 1;
                let col = pos % 5 + 1;
                key_coords.push(row * 10 + col);
            }
        }

        if key_coords.is_empty() {
            return "Error: Keyword cannot be empty".to_string();
        }

        match self.mode {
            PolybiusMode::Encode => {
                let mut result = Vec::new();
                let mut key_idx = 0;

                for c in input.to_uppercase().chars() {
                    if let Some(pos) = poly.find_in_square(&square, c) {
                        let row = pos / 5 + 1;
                        let col = pos % 5 + 1;
                        let val = row * 10 + col;

                        let key_val = key_coords[key_idx % key_coords.len()];
                        result.push((val + key_val).to_string());

                        key_idx += 1;
                    }
                }
                result.join(" ")
            }
            PolybiusMode::Decode => {
                let mut result = String::new();
                let mut key_idx = 0;

                let nums: Vec<&str> = input.split_whitespace().collect();
                for num_str in nums {
                    if let Ok(val) = num_str.parse::<usize>() {
                        let key_val = key_coords[key_idx % key_coords.len()];
                        if val > key_val {
                            let diff = val - key_val;
                            let row = diff / 10;
                            let col = diff % 10;

                            if row > 0 && col > 0 && row <= 5 && col <= 5 {
                                let pos = (row - 1) * 5 + (col - 1);
                                if pos < square.len() {
                                    result.push(square[pos]);
                                }
                            }
                        }
                        key_idx += 1;
                    }
                }
                result
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, PolybiusMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, PolybiusMode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("Polybius Key:");
            ui.text_edit_singleline(&mut self.polybius_key);
        });
        ui.horizontal(|ui| {
            ui.label("Keyword:");
            ui.text_edit_singleline(&mut self.keyword);
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct TapCodeModule {
    mode: PolybiusMode,
}

impl Default for TapCodeModule {
    fn default() -> Self {
        Self {
            mode: PolybiusMode::Encode,
        }
    }
}

impl Module for TapCodeModule {
    fn name(&self) -> &str {
        "Tap Code"
    }
    fn process(&self, input: &str) -> String {
        match self.mode {
            PolybiusMode::Encode => {
                // Tap code is basically Polybius square with dots
                let mut poly = PolybiusSquareModule::default();
                poly.mode = PolybiusMode::Encode;
                let coords = poly.process(input);
                coords
                    .chars()
                    .map(|c| {
                        if let Some(d) = c.to_digit(10) {
                            ".".repeat(d as usize) + " "
                        } else {
                            c.to_string()
                        }
                    })
                    .collect()
            }
            PolybiusMode::Decode => {
                // Count dots to get coordinates, then decode
                let mut coords = String::new();
                let groups: Vec<&str> = input.split_whitespace().collect();

                for group in groups {
                    let dot_count = group.chars().filter(|&c| c == '.').count();
                    if dot_count > 0 && dot_count <= 9 {
                        coords.push_str(&dot_count.to_string());
                    }
                }

                // Use Polybius decoder
                let mut poly = PolybiusSquareModule::default();
                poly.mode = PolybiusMode::Decode;
                poly.process(&coords)
            }
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, PolybiusMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, PolybiusMode::Decode, "Decode");
        });
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct TrifidCipherModule {
    key: String,
    mode: PolybiusMode,
}

impl Default for TrifidCipherModule {
    fn default() -> Self {
        Self {
            key: String::new(),
            mode: PolybiusMode::Encode,
        }
    }
}

impl Module for TrifidCipherModule {
    fn name(&self) -> &str {
        "Trifid Cipher"
    }

    fn process(&self, input: &str) -> String {
        // Generate 27-char square (A-Z + .)
        let mut square = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Add key chars
        for c in self.key.to_uppercase().chars() {
            if (c.is_ascii_alphabetic() || c == '.') && !seen.contains(&c) {
                square.push(c);
                seen.insert(c);
            }
        }

        // Add remaining chars
        for c in 'A'..='Z' {
            if !seen.contains(&c) {
                square.push(c);
                seen.insert(c);
            }
        }
        if !seen.contains(&'.') {
            square.push('.');
        }

        match self.mode {
            PolybiusMode::Encode => {
                let mut layers = Vec::new();
                let mut rows = Vec::new();
                let mut cols = Vec::new();

                // 1. Get coordinates (Layer, Row, Col)
                for c in input.to_uppercase().chars() {
                    if let Some(pos) = square.iter().position(|&x| x == c) {
                        layers.push(pos / 9);
                        rows.push((pos % 9) / 3);
                        cols.push(pos % 3);
                    }
                }

                // 2. Combine
                let mut combined = layers;
                combined.extend(rows);
                combined.extend(cols);

                // 3. Read triplets
                let mut result = String::new();
                for triplet in combined.chunks(3) {
                    if triplet.len() == 3 {
                        let pos = triplet[0] * 9 + triplet[1] * 3 + triplet[2];
                        if pos < square.len() {
                            result.push(square[pos]);
                        }
                    }
                }
                result
            }
            PolybiusMode::Decode => {
                let mut coords = Vec::new();
                for c in input.to_uppercase().chars() {
                    if let Some(pos) = square.iter().position(|&x| x == c) {
                        coords.push(pos / 9);
                        coords.push((pos % 9) / 3);
                        coords.push(pos % 3);
                    }
                }

                if coords.len() % 3 != 0 {
                    return "Error: Number of coordinates must be divisible by 3".to_string();
                }

                let third = coords.len() / 3;
                let layers = &coords[0..third];
                let rows = &coords[third..2 * third];
                let cols = &coords[2 * third..];

                let mut result = String::new();
                for i in 0..third {
                    let pos = layers[i] * 9 + rows[i] * 3 + cols[i];
                    if pos < square.len() {
                        result.push(square[pos]);
                    }
                }
                result
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, PolybiusMode::Encode, "Encode");
            ui.radio_value(&mut self.mode, PolybiusMode::Decode, "Decode");
        });
        ui.horizontal(|ui| {
            ui.label("Key:");
            ui.text_edit_singleline(&mut self.key);
        });
        ui.label("Note: Uses 27-char alphabet (A-Z + .)");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
