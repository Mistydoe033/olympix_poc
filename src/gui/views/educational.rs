use eframe::egui;
use crate::models::{AppState, View};

pub fn render_educational(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.button("← Back").clicked() {
            state.current_view = View::Home;
        }
        ui.heading("📚 Educational Content");
    });
    
    ui.add_space(10.0);
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        render_high_severity_section(ui);
        ui.add_space(20.0);
        render_medium_severity_section(ui);
        ui.add_space(20.0);
        render_low_severity_section(ui);
    });
}

fn render_high_severity_section(ui: &mut egui::Ui) {
    ui.heading("🔴 High Severity Vulnerabilities");
    ui.label("These vulnerabilities can lead to direct loss of funds or complete contract compromise.");
    
    ui.group(|ui| {
        ui.heading("Overflow/Underflow");
        ui.label("Arithmetic operations that can wrap around when they exceed the maximum value.");
        ui.label("Example: uint256 a = 2^256 - 1; a += 1; // Results in 0");
        ui.label("Fix: Use SafeMath or Solidity 0.8+");
    });
    
    ui.group(|ui| {
        ui.heading("Reentrancy");
        ui.label("External calls that allow attackers to re-enter the contract before state changes.");
        ui.label("Example: Sending ETH before updating balances");
        ui.label("Fix: Use reentrancy guards or checks-effects-interactions pattern");
    });
}

fn render_medium_severity_section(ui: &mut egui::Ui) {
    ui.heading("🟡 Medium Severity Vulnerabilities");
    ui.label("These vulnerabilities can lead to unexpected behavior or potential exploits.");
    
    ui.group(|ui| {
        ui.heading("Access Control");
        ui.label("Functions that can be called by unauthorized users.");
        ui.label("Fix: Add appropriate modifiers (onlyOwner, onlyRole, etc.)");
    });
    
    ui.group(|ui| {
        ui.heading("Unchecked External Calls");
        ui.label("Low-level calls that can fail silently.");
        ui.label("Fix: Always check return values and handle failures");
    });
}

fn render_low_severity_section(ui: &mut egui::Ui) {
    ui.heading("🟢 Low Severity Vulnerabilities");
    ui.label("These issues are less critical but should still be addressed.");
    
    ui.group(|ui| {
        ui.heading("Timestamp Dependence");
        ui.label("Using block.timestamp for critical decisions.");
        ui.label("Fix: Use block numbers or implement appropriate tolerances");
    });
} 