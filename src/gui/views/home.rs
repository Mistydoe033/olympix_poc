use eframe::egui;
use crate::models::{AppState, Action, View};
use crate::utils;
use crate::services;

pub fn render_home(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("🔒 Solidity Security Scanner");
    ui.add_space(10.0);
    
    ui.label("Welcome to the comprehensive Solidity vulnerability scanner!");
    ui.label("This tool helps identify common security issues in smart contracts.");
    ui.add_space(20.0);
    
    // Show error/success messages
    if let Some(error) = &state.error_message {
        ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
    }
    if let Some(success) = &state.success_message {
        ui.colored_label(egui::Color32::GREEN, format!("✅ {}", success));
    }
    
    // Navigation buttons
    ui.horizontal(|ui| {
        if ui.button("📚 Educational Content").clicked() {
            state.current_view = View::Educational;
        }
        
        if ui.button("📊 View Statistics").clicked() {
            show_statistics(ui, state);
        }
    });
    
    ui.add_space(20.0);
    
    // Contract list
    ui.heading("📄 Available Contracts");
    ui.add_space(10.0);
    
    if state.contracts.is_empty() {
        ui.label("No contracts found in the contracts directory.");
        ui.label("Please add some .sol files to get started.");
    } else {
        render_contract_list(ui, state);
    }
    
    ui.add_space(20.0);
    
    // Quick scan all button
    if ui.button("🚀 Scan All Contracts").clicked() {
        scan_all_contracts(state);
    }
}

fn show_statistics(ui: &mut egui::Ui, state: &AppState) {
    let total_contracts = state.contracts.len();
    let total_findings: usize = state.vulnerability_findings.len();
    let high_severity = state.vulnerability_findings.iter()
        .filter(|f| matches!(f.severity, crate::models::Severity::High))
        .count();
    
    ui.label(format!("📈 Total Contracts: {}", total_contracts));
    ui.label(format!("🔍 Total Findings: {}", total_findings));
    ui.label(format!("🔴 High Severity: {}", high_severity));
}

fn render_contract_list(ui: &mut egui::Ui, state: &mut AppState) {
    // Collect actions to perform after the loop
    let mut actions = Vec::new();
    
    for (i, contract) in state.contracts.iter().enumerate() {
        let contract_name = contract.name.clone();
        let contract_path = contract.path.clone();
        let contract_idx = i;
        
        ui.horizontal(|ui| {
            if ui.button(&contract_name).clicked() {
                actions.push(Action::SelectContract(contract_idx, contract_path));
            }
            
            if ui.button("🔍 Scan").clicked() {
                actions.push(Action::ScanContract(contract_idx));
            }
        });
    }
    
    // Execute actions after the loop
    for action in actions {
        match action {
            Action::SelectContract(idx, path) => {
                state.selected_contract = Some(idx);
                state.current_view = View::ContractDetail;
                match utils::read_file_content(&path) {
                    Ok(source) => {
                        state.contract_source = Some(source);
                        state.success_message = Some("Contract loaded successfully".to_string());
                    }
                    Err(e) => {
                        state.error_message = Some(format!("Failed to load contract: {}", e));
                    }
                }
            }
            Action::ScanContract(idx) => {
                state.selected_contract = Some(idx);
                scan_contract(state, idx);
            }
        }
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

fn scan_all_contracts(state: &mut AppState) {
    state.scan_in_progress = true;
    let mut all_findings = Vec::new();
    
    for contract in &state.contracts {
        if let Ok((findings, _)) = services::scan_contract_with_timing(&contract.path) {
            let mut findings_with_contract = findings;
            // Add contract name to findings
            for finding in &mut findings_with_contract {
                finding.snippet = format!("[{}] {}", contract.name, finding.snippet);
            }
            all_findings.extend(findings_with_contract);
        }
    }
    
    state.vulnerability_findings = all_findings;
    state.scan_in_progress = false;
    state.current_view = View::AnalysisResult;
    state.success_message = Some("All contracts scanned successfully".to_string());
} 