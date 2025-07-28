use eframe::egui;
use crate::models::{AppState, View};
use crate::services;
use crate::utils;

pub fn render_contract_detail(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.button("← Back").clicked() {
            state.current_view = View::Home;
        }
        
        if let Some(idx) = state.selected_contract {
            ui.heading(&format!("📄 {}", state.contracts[idx].name));
        }
    });
    
    ui.add_space(10.0);
    
    // Load contract source if not already loaded
    if state.contract_source.is_none() {
        if let Some(idx) = state.selected_contract {
            if let Ok(source) = utils::read_file_content(&state.contracts[idx].path) {
                state.contract_source = Some(source);
            } else {
                state.error_message = Some("Failed to read contract source".to_string());
                state.current_view = View::Home;
                return;
            }
        }
    }
    
    // Extract source and selected_contract before UI rendering to avoid borrowing conflicts
    let source_opt = state.contract_source.clone();
    let selected_contract = state.selected_contract;
    
    if let Some(source) = source_opt {
        let source_clone = source.clone();
        
        ui.horizontal(|ui| {
            if ui.button("🔍 Scan for Vulnerabilities").clicked() {
                if let Some(idx) = selected_contract {
                    scan_contract(state, idx);
                }
            }
            
            if ui.button("📋 Copy Source").clicked() {
                ui.output_mut(|o| o.copied_text = source_clone.clone());
                state.success_message = Some("Source code copied to clipboard".to_string());
            }
        });
        
        ui.add_space(10.0);
        
        // Display source code
        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
            ui.label(egui::RichText::new(&source).monospace());
        });
    }
}

fn scan_contract(state: &mut AppState, contract_idx: usize) {
    match services::scan_contract_with_timing(&state.contracts[contract_idx].path) {
        Ok((findings, scan_time)) => {
            state.vulnerability_findings = findings;
            state.last_scan_time = Some(scan_time);
            state.current_view = View::AnalysisResult;
            state.success_message = Some("Scan completed successfully".to_string());
        }
        Err(e) => {
            state.error_message = Some(format!("Failed to read contract: {}", e));
        }
    }
} 