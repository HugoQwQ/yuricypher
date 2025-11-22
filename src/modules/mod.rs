pub mod alphabet;
pub mod cipher;
pub mod encoding;
pub mod modern;
pub mod polybius;
pub mod transform;

use crate::module::Module;

pub fn create_module(id: &str) -> Option<Box<dyn Module>> {
    match id {
        "reverse" => Some(Box::new(transform::ReverseModule)),
        "case_transform" => Some(Box::new(transform::CaseTransformModule::default())),
        "replace" => Some(Box::new(transform::ReplaceModule::default())),
        "numeral" => Some(Box::new(transform::NumeralSystemModule::default())),
        "bitwise" => Some(Box::new(transform::BitwiseOperationModule::default())),
        "morse" => Some(Box::new(alphabet::MorseCodeModule::default())),
        "spelling" => Some(Box::new(alphabet::SpellingAlphabetModule)),
        "caesar" => Some(Box::new(cipher::CaesarCipherModule::default())),
        "rot13" => Some(Box::new(cipher::ROT13Module)),
        "a1z26" => Some(Box::new(cipher::A1Z26Module::default())),
        "affine" => Some(Box::new(cipher::AffineCipherModule::default())),
        "vigenere" => Some(Box::new(cipher::VigenereCipherModule::default())),
        "rail_fence" => Some(Box::new(cipher::RailFenceCipherModule::default())),
        "bacon" => Some(Box::new(cipher::BaconCipherModule::default())),
        "substitution" => Some(Box::new(cipher::AlphabeticalSubstitutionModule::default())),
        "polybius" => Some(Box::new(polybius::PolybiusSquareModule::default())),
        "adfgx" => Some(Box::new(polybius::ADFGXCipherModule)),
        "bifid" => Some(Box::new(polybius::BifidCipherModule)),
        "nihilist" => Some(Box::new(polybius::NihilistCipherModule)),
        "tap_code" => Some(Box::new(polybius::TapCodeModule)),
        "trifid" => Some(Box::new(polybius::TrifidCipherModule)),
        "base64" => Some(Box::new(encoding::Base64Module::default())),
        "base32" => Some(Box::new(encoding::Base32Module)),
        "ascii85" => Some(Box::new(encoding::Ascii85Module)),
        "baudot" => Some(Box::new(encoding::BaudotCodeModule)),
        "unicode" => Some(Box::new(encoding::UnicodeCodePointsModule::default())),
        "url" => Some(Box::new(encoding::UrlEncodingModule::default())),
        "punycode" => Some(Box::new(encoding::PunycodeModule)),
        "bootstring" => Some(Box::new(encoding::BootstringModule)),
        "integer" => Some(Box::new(encoding::IntegerModule::default())),
        "block_cipher" => Some(Box::new(modern::BlockCipherModule::default())),
        "rc4" => Some(Box::new(modern::RC4Module::default())),
        "hash" => Some(Box::new(modern::HashFunctionModule::default())),
        "hmac" => Some(Box::new(modern::HMACModule::default())),
        _ => None,
    }
}
