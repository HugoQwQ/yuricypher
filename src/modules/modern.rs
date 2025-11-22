use crate::module::Module;
use aes::Aes128;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};
use eframe::egui;
use md5::{Digest as Md5Digest, Md5};
use sha2::{Digest as Sha2Digest, Sha256};

type Aes128CbcEnc = Encryptor<Aes128>;
type Aes128CbcDec = Decryptor<Aes128>;

#[derive(PartialEq, Clone, Copy)]
enum BlockCipherMode {
    Encrypt,
    Decrypt,
}

pub struct BlockCipherModule {
    mode: BlockCipherMode,
    key: String,
    iv: String,
}

impl Default for BlockCipherModule {
    fn default() -> Self {
        Self {
            mode: BlockCipherMode::Encrypt,
            key: "0123456789abcdef".to_string(), // 16 bytes for AES-128
            iv: "fedcba9876543210".to_string(),  // 16 bytes IV
        }
    }
}

impl Module for BlockCipherModule {
    fn name(&self) -> &str {
        "Block Cipher (AES-128-CBC)"
    }

    fn process(&self, input: &str) -> String {
        // Ensure key and IV are exactly 16 bytes
        let mut key_bytes = [0u8; 16];
        let mut iv_bytes = [0u8; 16];

        let key_src = self.key.as_bytes();
        let iv_src = self.iv.as_bytes();

        for i in 0..16 {
            key_bytes[i] = *key_src.get(i).unwrap_or(&0);
            iv_bytes[i] = *iv_src.get(i).unwrap_or(&0);
        }

        match self.mode {
            BlockCipherMode::Encrypt => {
                let input_bytes = input.as_bytes();
                // Pad to multiple of 16 bytes (PKCS7 padding)
                let padding_len = 16 - (input_bytes.len() % 16);
                let mut buffer = input_bytes.to_vec();
                buffer.extend(vec![padding_len as u8; padding_len]);

                // Ensure buffer is large enough
                let len = buffer.len();
                buffer.resize(len + 16, 0); // Add extra space for padding

                let cipher = Aes128CbcEnc::new(&key_bytes.into(), &iv_bytes.into());
                match cipher
                    .encrypt_padded_mut::<cbc::cipher::block_padding::NoPadding>(&mut buffer, len)
                {
                    Ok(ciphertext) => hex::encode(ciphertext),
                    Err(_) => "Encryption error".to_string(),
                }
            }
            BlockCipherMode::Decrypt => {
                // Decode hex input
                let mut ciphertext = match hex::decode(input.trim()) {
                    Ok(ct) => ct,
                    Err(_) => return "Invalid hex input".to_string(),
                };

                let cipher = Aes128CbcDec::new(&key_bytes.into(), &iv_bytes.into());
                match cipher
                    .decrypt_padded_mut::<cbc::cipher::block_padding::NoPadding>(&mut ciphertext)
                {
                    Ok(plaintext) => {
                        // Remove PKCS7 padding
                        let mut pt = plaintext.to_vec();
                        if let Some(&padding_len) = pt.last() {
                            if padding_len > 0 && padding_len <= 16 {
                                let new_len = pt.len().saturating_sub(padding_len as usize);
                                pt.truncate(new_len);
                            }
                        }
                        String::from_utf8_lossy(&pt).to_string()
                    }
                    Err(_) => "Decryption error".to_string(),
                }
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, BlockCipherMode::Encrypt, "Encrypt");
            ui.radio_value(&mut self.mode, BlockCipherMode::Decrypt, "Decrypt");
        });
        ui.horizontal(|ui| {
            ui.label("Key (16 bytes):");
            ui.text_edit_singleline(&mut self.key);
        });
        ui.horizontal(|ui| {
            ui.label("IV (16 bytes):");
            ui.text_edit_singleline(&mut self.iv);
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
enum RC4Mode {
    Encrypt,
    Decrypt,
}

pub struct RC4Module {
    mode: RC4Mode,
    key: String,
}

impl Default for RC4Module {
    fn default() -> Self {
        Self {
            mode: RC4Mode::Encrypt,
            key: "secret".to_string(),
        }
    }
}

impl RC4Module {
    fn rc4_keystream(&self, length: usize) -> Vec<u8> {
        let key_bytes = self.key.as_bytes();
        let mut s: Vec<u8> = (0..=255).collect();

        // KSA (Key Scheduling Algorithm)
        let mut j: u8 = 0;
        for i in 0..256 {
            j = j
                .wrapping_add(s[i])
                .wrapping_add(key_bytes[i % key_bytes.len()]);
            s.swap(i, j as usize);
        }

        // PRGA (Pseudo-Random Generation Algorithm)
        let mut keystream = Vec::with_capacity(length);
        let mut i: u8 = 0;
        let mut j: u8 = 0;

        for _ in 0..length {
            i = i.wrapping_add(1);
            j = j.wrapping_add(s[i as usize]);
            s.swap(i as usize, j as usize);
            let k = s[(s[i as usize].wrapping_add(s[j as usize])) as usize];
            keystream.push(k);
        }

        keystream
    }
}

impl Module for RC4Module {
    fn name(&self) -> &str {
        "RC4"
    }

    fn process(&self, input: &str) -> String {
        match self.mode {
            RC4Mode::Encrypt => {
                let input_bytes = input.as_bytes();
                let keystream = self.rc4_keystream(input_bytes.len());
                let ciphertext: Vec<u8> = input_bytes
                    .iter()
                    .zip(keystream.iter())
                    .map(|(a, b)| a ^ b)
                    .collect();
                hex::encode(ciphertext)
            }
            RC4Mode::Decrypt => {
                // Decode hex input
                let ciphertext = match hex::decode(input.trim()) {
                    Ok(ct) => ct,
                    Err(_) => return "Invalid hex input".to_string(),
                };

                let keystream = self.rc4_keystream(ciphertext.len());
                let plaintext: Vec<u8> = ciphertext
                    .iter()
                    .zip(keystream.iter())
                    .map(|(a, b)| a ^ b)
                    .collect();
                String::from_utf8_lossy(&plaintext).to_string()
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, RC4Mode::Encrypt, "Encrypt");
            ui.radio_value(&mut self.mode, RC4Mode::Decrypt, "Decrypt");
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

#[derive(PartialEq, Clone, Copy)]
enum HashAlgorithm {
    MD5,
    SHA256,
}

pub struct HashFunctionModule {
    algorithm: HashAlgorithm,
}

impl Default for HashFunctionModule {
    fn default() -> Self {
        Self {
            algorithm: HashAlgorithm::SHA256,
        }
    }
}

impl Module for HashFunctionModule {
    fn name(&self) -> &str {
        "Hash Function"
    }

    fn process(&self, input: &str) -> String {
        match self.algorithm {
            HashAlgorithm::MD5 => {
                let mut hasher = Md5::new();
                hasher.update(input.as_bytes());
                format!("{:x}", hasher.finalize())
            }
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                hasher.update(input.as_bytes());
                format!("{:x}", hasher.finalize())
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Algorithm:");
            ui.radio_value(&mut self.algorithm, HashAlgorithm::MD5, "MD5");
            ui.radio_value(&mut self.algorithm, HashAlgorithm::SHA256, "SHA256");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct HMACModule {
    key: String,
    algorithm: HashAlgorithm,
}

impl Default for HMACModule {
    fn default() -> Self {
        Self {
            key: String::from("secret"),
            algorithm: HashAlgorithm::SHA256,
        }
    }
}

impl Module for HMACModule {
    fn name(&self) -> &str {
        "HMAC"
    }

    fn process(&self, input: &str) -> String {
        // Simple HMAC implementation
        let key_bytes = self.key.as_bytes();
        let block_size = 64; // For both MD5 and SHA256

        let mut key_padded = vec![0u8; block_size];
        if key_bytes.len() <= block_size {
            key_padded[..key_bytes.len()].copy_from_slice(key_bytes);
        } else {
            // Hash the key if it's too long
            match self.algorithm {
                HashAlgorithm::MD5 => {
                    let mut hasher = Md5::new();
                    hasher.update(key_bytes);
                    let result = hasher.finalize();
                    key_padded[..result.len()].copy_from_slice(&result);
                }
                HashAlgorithm::SHA256 => {
                    let mut hasher = Sha256::new();
                    hasher.update(key_bytes);
                    let result = hasher.finalize();
                    key_padded[..result.len()].copy_from_slice(&result);
                }
            }
        }

        let mut o_key_pad = vec![0x5c; block_size];
        let mut i_key_pad = vec![0x36; block_size];

        for i in 0..block_size {
            o_key_pad[i] ^= key_padded[i];
            i_key_pad[i] ^= key_padded[i];
        }

        // Inner hash
        let mut inner_data = i_key_pad;
        inner_data.extend_from_slice(input.as_bytes());

        let inner_hash = match self.algorithm {
            HashAlgorithm::MD5 => {
                let mut hasher = Md5::new();
                hasher.update(&inner_data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                hasher.update(&inner_data);
                hasher.finalize().to_vec()
            }
        };

        // Outer hash
        let mut outer_data = o_key_pad;
        outer_data.extend_from_slice(&inner_hash);

        match self.algorithm {
            HashAlgorithm::MD5 => {
                let mut hasher = Md5::new();
                hasher.update(&outer_data);
                format!("{:x}", hasher.finalize())
            }
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                hasher.update(&outer_data);
                format!("{:x}", hasher.finalize())
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Key:");
            ui.text_edit_singleline(&mut self.key);
        });
        ui.horizontal(|ui| {
            ui.label("Algorithm:");
            ui.radio_value(&mut self.algorithm, HashAlgorithm::MD5, "MD5");
            ui.radio_value(&mut self.algorithm, HashAlgorithm::SHA256, "SHA256");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
