use egui;
use crate::generator::PassSymbols;

pub fn toggle(ui: &mut egui::Ui, set: &mut Vec<PassSymbols>, sym: PassSymbols, label: &str) {
    let enabled = set.contains(&sym);

    let button = egui::Button::new(label)
        .fill(if enabled {
            egui::Color32::DARK_GREEN
        } else {
            egui::Color32::DARK_RED
        });

    if ui.add(button).clicked() {
        if enabled {
            if let Some(pos) = set.iter().position(|x| *x == sym) {
                set.remove(pos);
            }
        } else {
            set.push(sym);
        }
    }
}
