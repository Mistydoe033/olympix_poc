use eframe::egui;
use crate::models::{AppState, FileType, View};
use crate::utils;
use rfd::FileDialog;

pub fn render_contracts_tab(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.heading(egui::RichText::new("Smart Contracts")
            .size(20.0)
            .strong());
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(egui::RichText::new("📁 Import Files")
                .size(14.0)
                .color(egui::Color32::WHITE))
                .clicked() {
                // Open file dialog
                if let Some(files) = FileDialog::new()
                    .add_filter("Solidity", &["sol"])
                    .set_directory(".")
                    .pick_files() {
                    let mut imported = 0;
                    for file in files {
                        if let Ok(_) = utils::import_file(&file, &FileType::Contract) {
                            imported += 1;
                        }
                    }
                    state.contracts = utils::list_contracts();
                    if imported > 0 {
                        state.success_message = Some(format!("Imported {} contract(s)", imported));
                    } else {
                        state.error_message = Some("No contracts imported".to_string());
                    }
                }
            }
            
            if ui.button(egui::RichText::new("📂 Import Folder")
                .size(14.0)
                .color(egui::Color32::WHITE))
                .clicked() {
                // Open folder dialog
                if let Some(folder) = FileDialog::new().set_directory(".").pick_folder() {
                    match utils::import_folder(&folder, &FileType::Contract) {
                        Ok(imported) => {
                            state.contracts = utils::list_contracts();
                            if !imported.is_empty() {
                                state.success_message = Some(format!("Imported {} contract(s) from folder", imported.len()));
                            } else {
                                state.error_message = Some("No contracts imported from folder".to_string());
                            }
                        }
                        Err(e) => {
                            state.error_message = Some(format!("Failed to import folder: {}", e));
                        }
                    }
                }
            }
        });
    });
    
    ui.add_space(10.0);
    
    // Statistics card
    render_statistics_card(ui, state);
    
    ui.add_space(20.0);
    
    // Contracts list
    if state.contracts.is_empty() {
        render_empty_state(ui, state);
    } else {
        render_contracts_list(ui, state);
    }
}

fn render_statistics_card(ui: &mut egui::Ui, state: &AppState) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("📊 Statistics")
                .size(16.0)
                .strong());
        });
        
        ui.add_space(5.0);
        
        ui.horizontal(|ui| {
            ui.label(format!("Total Contracts: {}", state.contracts.len()));
            ui.label(format!("Total Size: {}", 
                utils::format_file_size(state.contracts.iter().map(|c| c.size).sum())));
        });
        
        if let Some(selected) = state.selected_contract {
            if let Some(contract) = state.contracts.get(selected) {
                ui.add_space(5.0);
                ui.label(format!("Selected: {} ({})", 
                    contract.name, 
                    utils::format_file_size(contract.size)));
            }
        }
    });
}

fn render_empty_state(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.label(egui::RichText::new("📄 No Contracts Found")
            .size(24.0)
            .strong()
            .color(egui::Color32::GRAY));
        
        ui.add_space(10.0);
        
        ui.label(egui::RichText::new("Import your Solidity contracts to get started")
            .size(16.0)
            .color(egui::Color32::GRAY));
        
        ui.add_space(20.0);
        
        if ui.button(egui::RichText::new("📁 Import Your First Contract")
            .size(16.0)
            .color(egui::Color32::WHITE))
            .clicked() {
            // Open file dialog
            if let Some(files) = FileDialog::new()
                .add_filter("Solidity", &["sol"])
                .set_directory(".")
                .pick_files() {
                let mut imported = 0;
                for file in files {
                    if let Ok(_) = utils::import_file(&file, &FileType::Contract) {
                        imported += 1;
                    }
                }
                state.contracts = utils::list_contracts();
                if imported > 0 {
                    state.success_message = Some(format!("Imported {} contract(s)", imported));
                } else {
                    state.error_message = Some("No contracts imported".to_string());
                }
            }
        }
        
        ui.add_space(20.0);
        
        ui.label(egui::RichText::new("Supported formats: .sol files")
            .size(14.0)
            .color(egui::Color32::GRAY));
    });
}

fn render_contracts_list(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading(egui::RichText::new("Your Contracts")
        .size(18.0)
        .strong());
    
    ui.add_space(10.0);
    
    egui::ScrollArea::vertical().id_source("contracts_list_scroll").max_height(400.0).show(ui, |ui| {
        for (i, contract) in state.contracts.iter().enumerate() {
            let is_selected = state.selected_contract == Some(i);
            
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    // Selection indicator
                    if is_selected {
                        ui.label(egui::RichText::new("▶")
                            .color(egui::Color32::from_rgb(59, 130, 246))
                            .strong());
                    } else {
                        ui.label("  ");
                    }
                    
                    // Contract info
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&contract.name)
                                .size(16.0)
                                .strong());
                            
                            if is_selected {
                                ui.label(egui::RichText::new("(Selected)")
                                    .color(egui::Color32::from_rgb(59, 130, 246))
                                    .size(12.0));
                            }
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label(format!("Size: {}", utils::format_file_size(contract.size)));
                            ui.label("•");
                            ui.label(format!("Modified: {}", utils::format_timestamp(contract.last_modified)));
                        });
                    });
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new("🔍 Scan")
                            .size(12.0)
                            .color(egui::Color32::WHITE))
                            .clicked() {
                            // Run scan on this contract
                            match crate::services::scan_contract_with_timing(&contract.path) {
                                Ok((findings, scan_time)) => {
                                    state.vulnerability_findings = findings;
                                    state.last_scan_time = Some(scan_time);
                                    state.current_view = crate::models::View::AnalysisResult;
                                    state.success_message = Some("Scan completed successfully".to_string());
                                }
                                Err(e) => {
                                    state.error_message = Some(format!("Failed to scan contract: {}", e));
                                }
                            }
                        }
                        
                        if ui.button(egui::RichText::new("👁 View")
                            .size(12.0)
                            .color(egui::Color32::WHITE))
                            .clicked() {
                            state.selected_contract = Some(i);
                            state.current_view = crate::models::View::ContractDetail;
                        }
                    });
                });
            });
            
            ui.add_space(5.0);
        }
    });
} 