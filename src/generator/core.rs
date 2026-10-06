use rand::Rng;

pub struct GenPassCore;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PassSymbols {
    Lower,
    Upper,
    Digits,
    Special,
}

impl PassSymbols {
    pub fn bytes(&self) -> &'static [u8] {
        match self {
            PassSymbols::Lower => b"abcdefghijklmnopqrstuvwxyz",
            PassSymbols::Upper => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            PassSymbols::Digits => b"0123456789",
            PassSymbols::Special => b"!@#$%^&*()",
        }
    }
}

impl GenPassCore {
    pub fn generate(length: usize, symbols: Vec<PassSymbols>) -> String {
        let mut rng = rand::rng();

        let pool: Vec<u8> = symbols
            .iter()
            .flat_map(|s| s.bytes())
            .copied()
            .collect();
    
        assert!(!pool.is_empty(), "symbol pool is empty");
        
        let mut password = String::with_capacity(length);
        for _ in 0..length {
            password
                .push(pool[rng.random_range(0..pool.len())] as char);
        }
        password
    }
}