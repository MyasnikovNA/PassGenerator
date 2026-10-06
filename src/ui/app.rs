use eframe::egui;
use crate::generator::GenPassCore;
use crate::generator::PassSymbols;
use crate::ui::buttons::toggle;
use crate::ui::default::GenPassApp;
use arboard;

impl eframe::App for GenPassApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Really Simple Password Generator");
            
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Password length");
                ui.add(egui::Slider::new(&mut self.length, 4..=64));
                ui.label(format!("{}", self.length));
            });
            
            ui.separator();

            ui.horizontal(|ui| {
                toggle(ui, &mut self.selected, PassSymbols::Lower, "Lowercase");
                toggle(ui, &mut self.selected, PassSymbols::Upper, "Uppercase");
                toggle(ui, &mut self.selected, PassSymbols::Digits, "Digits");
                toggle(ui, &mut self.selected, PassSymbols::Special, "Symbols");
            });            
            
            ui.separator();
            
            ui.label("Generated password:");
            ui.text_edit_singleline(&mut self.password);
            
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                if ui.button("Generate password").clicked() {
                    match self.selected.is_empty() {
                        true => self.password = "ErrorNoSymbols".into(),
                        false => {
                            let symbols: Vec<_> = self.selected.iter().copied().collect();
                            self.password = GenPassCore::generate(self.length, symbols);
                        }
                    }
                } 
                
                ui.add_space(2.0);

                if ui.button("Copy Password").clicked() {
                    match arboard::Clipboard::new() {
                        Ok(mut clipboard) => {
                            match clipboard.set_text(self.password.clone()) {
                                Ok(_) => {},
                                Err(e) => eprintln!("Failed to copy to clipboard: {:?}", e),
                            }
                        }
                        Err(e) => eprintln!("Failed to create clipboard: {:?}", e),
                    }
                }
            });
        });
    }
}        