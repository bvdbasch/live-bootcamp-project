use rust_qrng::QuantumRNG;
use unicode_segmentation::UnicodeSegmentation;

const ALPHANUMERIC: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct SecureRandomAlphaNum {
    quantum_rng: crate::QuantumRNG,
}

impl SecureRandomAlphaNum {
    fn new() -> Self {
        SecureRandomAlphaNum {
            quantum_rng: crate::QuantumRNG::new(),
        }
    }

    pub fn get_random_position(&mut self) -> u8 {
        let upper_index = (ALPHANUMERIC.len() - 1) as u64;
        return self.quantum_rng.generate_range_u64(0, upper_index) as u8;
    }

    pub fn get_random_alphanumeric(&mut self) -> &str {
        let index = self.get_random_position() as usize;
        let graphemes: Vec<&str> = ALPHANUMERIC.graphemes(true).collect();
        return graphemes[index];
    }
}

fn main() {
    let mut secure_random_alpha_num = SecureRandomAlphaNum::new();
    let mut pass = String::new();

    for i in 0..23 {
        match i {
            4 | 9 | 14 | 19 => {
                pass += "-";
            }
            _ => {
                pass += secure_random_alpha_num.get_random_alphanumeric();
            }
        };
    }

    println!("{pass}");
}
