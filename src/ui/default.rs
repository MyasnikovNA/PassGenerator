use eframe::CreationContext;
use crate::generator::PassSymbols;


pub struct GenPassApp {
    pub password: String,
    pub length: usize,
    pub selected: Vec<PassSymbols>,
}

impl Default for GenPassApp {
    fn default() -> Self {
        Self {
            password: String::new(),
            length: 12,
            selected: vec![PassSymbols::Lower, PassSymbols::Upper, PassSymbols::Digits, PassSymbols::Special],
        }
    }
}

impl GenPassApp {
    pub fn new(_: &CreationContext<'_>) -> Self {
        Self::default()
    }
}