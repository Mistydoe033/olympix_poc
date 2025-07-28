use eframe::egui;
use crate::models::AppState;
use crate::utils;

pub fn render_pattern_detail(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.button("← Back").clicked() {
            state.show_pattern_view = false;
            state.selected_pattern = None;
            state.pattern_source = None;
        }
        
        if let Some(idx) = state.selected_pattern {
            ui.heading(&format!("🔍 {}", state.patterns[idx].name));
        }
    });
    
    ui.add_space(10.0);
    
    // Load pattern source if not already loaded
    if state.pattern_source.is_none() {
        if let Some(idx) = state.selected_pattern {
            if let Ok(source) = utils::read_file_content(&state.patterns[idx].path) {
                state.pattern_source = Some(source);
            } else {
                state.error_message = Some("Failed to read pattern source".to_string());
                state.show_pattern_view = false;
                return;
            }
        }
    }
    
    // Extract source and selected_pattern before UI rendering to avoid borrowing conflicts
    let source_opt = state.pattern_source.clone();
    let selected_pattern = state.selected_pattern;
    
    if let Some(source) = source_opt {
        let source_clone = source.clone();
        
        ui.horizontal(|ui| {
            if ui.button("📋 Copy Pattern").clicked() {
                ui.output_mut(|o| o.copied_text = source_clone.clone());
                state.success_message = Some("Pattern copied to clipboard".to_string());
            }
            
            if let Some(idx) = selected_pattern {
                let pattern = &state.patterns[idx];
                ui.label(format!("Type: {}", match pattern.pattern_type {
                    crate::models::FileType::RustPattern => "Rust Pattern",
                    crate::models::FileType::RegexPattern => "Regex Pattern",
                    _ => "Unknown",
                }));
            }
        });
        
        ui.add_space(10.0);
        
        // Display pattern content
        ui.label(egui::RichText::new("Pattern Content:")
            .size(16.0)
            .strong());
        
        ui.add_space(5.0);
        
        egui::ScrollArea::vertical().id_source("pattern_source_scroll").max_height(400.0).show(ui, |ui| {
            ui.label(egui::RichText::new(&source).monospace());
        });
    }
} 