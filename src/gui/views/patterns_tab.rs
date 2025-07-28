use eframe::egui;
use crate::models::{AppState, FileType};
use crate::utils;
use rfd::FileDialog;

pub fn render_patterns_tab(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.heading(egui::RichText::new("Detection Patterns")
            .size(20.0)
            .strong());
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(egui::RichText::new("📁 Import Rust Pattern")
                .size(14.0)
                .color(egui::Color32::WHITE))
                .clicked() {
                // Open file dialog for Rust patterns
                if let Some(files) = FileDialog::new()
                    .add_filter("Rust", &["rs"])
                    .set_directory(".")
                    .pick_files() {
                    let mut imported = 0;
                    for file in files {
                        if let Ok(_) = utils::import_file(&file, &FileType::RustPattern) {
                            imported += 1;
                        }
                    }
                    state.patterns = utils::list_patterns();
                    if imported > 0 {
                        state.success_message = Some(format!("Imported {} Rust pattern(s)", imported));
                    } else {
                        state.error_message = Some("No Rust patterns imported".to_string());
                    }
                }
            }
            
            if ui.button(egui::RichText::new("📁 Import Regex Pattern")
                .size(14.0)
                .color(egui::Color32::WHITE))
                .clicked() {
                // Open file dialog for Regex patterns
                if let Some(files) = FileDialog::new()
                    .add_filter("Regex", &["regex", "txt"])
                    .set_directory(".")
                    .pick_files() {
                    let mut imported = 0;
                    for file in files {
                        if let Ok(_) = utils::import_file(&file, &FileType::RegexPattern) {
                            imported += 1;
                        }
                    }
                    state.patterns = utils::list_patterns();
                    if imported > 0 {
                        state.success_message = Some(format!("Imported {} Regex pattern(s)", imported));
                    } else {
                        state.error_message = Some("No Regex patterns imported".to_string());
                    }
                }
            }
        });
    });
    
    ui.add_space(10.0);
    
    // Pattern types overview
    render_pattern_types_overview(ui);
    
    ui.add_space(20.0);
    
    // Patterns list
    if state.patterns.is_empty() {
        render_empty_patterns_state(ui);
    } else {
        render_patterns_list(ui, state);
    }
}

fn render_pattern_types_overview(ui: &mut egui::Ui) {
    ui.group(|ui| {
        ui.label(egui::RichText::new("🔍 Pattern Types")
            .size(16.0)
            .strong());
        
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            // Rust Patterns
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("🦀 Rust Patterns")
                    .size(14.0)
                    .strong()
                    .color(egui::Color32::from_rgb(222, 165, 132)));
                ui.label("Advanced vulnerability detection using Rust algorithms");
                ui.label("• Syntax-aware analysis");
                ui.label("• Context-sensitive detection");
                ui.label("• Custom logic implementation");
            });
            
            ui.add_space(20.0);
            
            // Regex Patterns
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("🔤 Regex Patterns")
                    .size(14.0)
                    .strong()
                    .color(egui::Color32::from_rgb(147, 197, 253)));
                ui.label("Fast pattern matching using regular expressions");
                ui.label("• Quick text-based detection");
                ui.label("• Flexible pattern matching");
                ui.label("• Easy to create and modify");
            });
        });
    });
}

fn render_empty_patterns_state(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.label(egui::RichText::new("🔍 No Patterns Found")
            .size(24.0)
            .strong()
            .color(egui::Color32::GRAY));
        
        ui.add_space(10.0);
        
        ui.label(egui::RichText::new("Import detection patterns to enhance vulnerability scanning")
            .size(16.0)
            .color(egui::Color32::GRAY));
        
        ui.add_space(20.0);
        
        ui.horizontal(|ui| {
            if ui.button(egui::RichText::new("🦀 Import Rust Pattern")
                .size(16.0)
                .color(egui::Color32::WHITE))
                .clicked() {
                // TODO: Open file dialog
            }
            
            ui.add_space(10.0);
            
            if ui.button(egui::RichText::new("🔤 Import Regex Pattern")
                .size(16.0)
                .color(egui::Color32::WHITE))
                .clicked() {
                // TODO: Open file dialog
            }
        });
        
        ui.add_space(20.0);
        
        ui.label(egui::RichText::new("Supported formats: .rs (Rust), .regex, .txt (Regex)")
            .size(14.0)
            .color(egui::Color32::GRAY));
    });
}

fn render_patterns_list(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading(egui::RichText::new("Your Patterns")
        .size(18.0)
        .strong());
    
    ui.add_space(10.0);
    
    // Group patterns by type - clone to avoid borrowing issues
    let rust_patterns: Vec<_> = state.patterns.iter()
        .filter(|p| matches!(p.pattern_type, FileType::RustPattern))
        .cloned()
        .collect();
    
    let regex_patterns: Vec<_> = state.patterns.iter()
        .filter(|p| matches!(p.pattern_type, FileType::RegexPattern))
        .cloned()
        .collect();
    
    // Rust Patterns Section
    if !rust_patterns.is_empty() {
        ui.label(egui::RichText::new("🦀 Rust Patterns")
            .size(16.0)
            .strong()
            .color(egui::Color32::from_rgb(222, 165, 132)));
        
        ui.add_space(5.0);
        
        egui::ScrollArea::vertical().id_source("rust_patterns_scroll").max_height(200.0).show(ui, |ui| {
            for pattern in &rust_patterns {
                render_pattern_item(ui, pattern, state);
            }
        });
        
        ui.add_space(15.0);
    }
    
    // Regex Patterns Section
    if !regex_patterns.is_empty() {
        ui.label(egui::RichText::new("🔤 Regex Patterns")
            .size(16.0)
            .strong()
            .color(egui::Color32::from_rgb(147, 197, 253)));
        
        ui.add_space(5.0);
        
        egui::ScrollArea::vertical().id_source("regex_patterns_scroll").max_height(200.0).show(ui, |ui| {
            for pattern in &regex_patterns {
                render_pattern_item(ui, pattern, state);
            }
        });
    }
}

fn render_pattern_item(ui: &mut egui::Ui, pattern: &crate::models::PatternInfo, state: &mut AppState) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            // Pattern icon
            let icon = match pattern.pattern_type {
                FileType::RustPattern => "🦀",
                FileType::RegexPattern => "🔤",
                _ => "📄",
            };
            ui.label(icon);
            
            // Pattern info
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&pattern.name)
                        .size(14.0)
                        .strong());
                    
                    // Type badge
                    let badge_color = match pattern.pattern_type {
                        FileType::RustPattern => egui::Color32::from_rgb(222, 165, 132),
                        FileType::RegexPattern => egui::Color32::from_rgb(147, 197, 253),
                        _ => egui::Color32::GRAY,
                    };
                    
                    ui.label(egui::RichText::new(match pattern.pattern_type {
                        FileType::RustPattern => "RUST",
                        FileType::RegexPattern => "REGEX",
                        _ => "UNKNOWN",
                    })
                    .size(10.0)
                    .color(badge_color)
                    .background_color(badge_color.linear_multiply(0.2)));
                });
                
                ui.label(egui::RichText::new(&pattern.description)
                    .size(12.0)
                    .color(egui::Color32::GRAY));
                
                ui.horizontal(|ui| {
                    ui.label(format!("Size: {}", utils::format_file_size(pattern.size)));
                    ui.label("•");
                    ui.label(format!("Modified: {}", utils::format_timestamp(pattern.last_modified)));
                });
            });
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(egui::RichText::new("👁 View")
                    .size(12.0)
                    .color(egui::Color32::WHITE))
                    .clicked() {
                    // Set pattern for viewing
                    state.selected_pattern = Some(state.patterns.iter().position(|p| p.name == pattern.name).unwrap_or(0));
                    state.show_pattern_view = true;
                    state.pattern_source = None; // Will be loaded in pattern detail view
                }
                
                if ui.button(egui::RichText::new("⚙️ Edit")
                    .size(12.0)
                    .color(egui::Color32::WHITE))
                    .clicked() {
                    // TODO: Implement pattern editing
                    // For now, show a success message
                    state.success_message = Some("Pattern editing coming soon!".to_string());
                }
            });
        });
    });
    
    ui.add_space(5.0);
} 