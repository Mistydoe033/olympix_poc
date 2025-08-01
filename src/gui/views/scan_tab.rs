use eframe::egui;
use crate::models::{AppState, Severity};
use crate::services;
use crate::utils;

pub fn render_scan_tab(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.heading(egui::RichText::new("Vulnerability Scanner")
            .size(20.0)
            .strong());
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(egui::RichText::new("🔍 Scan All Contracts")
                .size(14.0)
                .color(egui::Color32::WHITE))
                .clicked() {
                scan_all_contracts(state);
            }
        });
    });
    
    ui.add_space(10.0);
    
    // Scan summary card
    render_scan_summary_card(ui, state);
    
    ui.add_space(20.0);
    
    // Scan results
    if state.vulnerability_findings.is_empty() && !state.scan_in_progress {
        render_no_scan_results(ui);
    } else if state.scan_in_progress {
        render_scan_progress(ui);
    } else {
        render_scan_results(ui, state);
    }
}

fn render_scan_summary_card(ui: &mut egui::Ui, state: &AppState) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("📊 Scan Summary")
                .size(16.0)
                .strong());
            
            if let Some(scan_time) = state.last_scan_time {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("⏱️ Last scan: {:.2}s", scan_time));
                });
            }
        });
        
        ui.add_space(10.0);
        
        let summary = &state.scan_results_summary;
        
        ui.horizontal(|ui| {
            // Critical findings
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(format!("{}", summary.critical_findings))
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::RED));
                ui.label("Critical");
            });
            
            ui.add_space(20.0);
            
            // High findings
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(format!("{}", summary.high_findings))
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::from_rgb(245, 158, 11)));
                ui.label("High");
            });
            
            ui.add_space(20.0);
            
            // Medium findings
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(format!("{}", summary.medium_findings))
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::YELLOW));
                ui.label("Medium");
            });
            
            ui.add_space(20.0);
            
            // Low findings
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(format!("{}", summary.low_findings))
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::GREEN));
                ui.label("Low");
            });
            
            ui.add_space(20.0);
            
            // Total contracts
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(format!("{}", summary.total_contracts))
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::from_rgb(59, 130, 246)));
                ui.label("Contracts");
            });
        });
        
        if !summary.contracts_scanned.is_empty() {
            ui.add_space(10.0);
            ui.label(egui::RichText::new("Scanned Contracts:")
                .size(12.0)
                .strong());
            for contract in &summary.contracts_scanned {
                ui.label(format!("• {}", contract));
            }
        }
    });
}

fn render_no_scan_results(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.label(egui::RichText::new("🔍 No Scan Results")
            .size(24.0)
            .strong()
            .color(egui::Color32::GRAY));
        
        ui.add_space(10.0);
        
        ui.label(egui::RichText::new("Run a vulnerability scan to see results")
            .size(16.0)
            .color(egui::Color32::GRAY));
        
        ui.add_space(20.0);
        
        if ui.button(egui::RichText::new("🚀 Start Your First Scan")
            .size(16.0)
            .color(egui::Color32::WHITE))
            .clicked() {
            // TODO: Trigger scan
        }
        
        ui.add_space(20.0);
        
        ui.label(egui::RichText::new("Make sure you have contracts imported first")
            .size(14.0)
            .color(egui::Color32::GRAY));
    });
}

fn render_scan_progress(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.label(egui::RichText::new("🔍 Scanning Contracts...")
            .size(24.0)
            .strong()
            .color(egui::Color32::from_rgb(59, 130, 246)));
        
        ui.add_space(20.0);
        
        // Progress bar
        ui.add(egui::ProgressBar::new(0.5).animate(true));
        
        ui.add_space(10.0);
        
        ui.label("Analyzing smart contracts for vulnerabilities...");
        ui.label("This may take a few moments depending on the number of contracts.");
    });
}

fn render_scan_results(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.heading(egui::RichText::new("Scan Results")
            .size(18.0)
            .strong());
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Vulnerability type filter
            ui.label("Type:");
            if ui.button("All Types").clicked() {
                state.selected_vulnerability_type = None;
            }
            if ui.button("🔴 Overflow").clicked() {
                state.selected_vulnerability_type = Some(crate::models::VulnerabilityType::Overflow);
            }
            if ui.button("🟡 Reentrancy").clicked() {
                state.selected_vulnerability_type = Some(crate::models::VulnerabilityType::Reentrancy);
            }
            if ui.button("🟠 Access Control").clicked() {
                state.selected_vulnerability_type = Some(crate::models::VulnerabilityType::AccessControl);
            }
            if ui.button("🟢 External Call").clicked() {
                state.selected_vulnerability_type = Some(crate::models::VulnerabilityType::UncheckedExternalCall);
            }
            
            ui.add_space(20.0);
            
            // Severity filter
            ui.label("Severity:");
            if ui.button("All").clicked() {
                state.selected_severity = None;
            }
            if ui.button("🔴 Critical").clicked() {
                state.selected_severity = Some(Severity::Critical);
            }
            if ui.button("🟡 High").clicked() {
                state.selected_severity = Some(Severity::High);
            }
            if ui.button("🟠 Medium").clicked() {
                state.selected_severity = Some(Severity::Medium);
            }
            if ui.button("🟢 Low").clicked() {
                state.selected_severity = Some(Severity::Low);
            }
        });
    });
    
    ui.add_space(10.0);
    
    // Filter findings - clone to avoid borrowing issues
    let filtered_findings: Vec<crate::models::VulnerabilityFinding> = state.vulnerability_findings.iter()
        .filter(|f| {
            // Apply severity filter if selected
            let severity_match = if let Some(severity) = &state.selected_severity {
                std::mem::discriminant(&f.severity) == std::mem::discriminant(severity)
            } else {
                true
            };
            
            // Apply vulnerability type filter if selected
            let type_match = if let Some(vuln_type) = &state.selected_vulnerability_type {
                std::mem::discriminant(&f.category) == std::mem::discriminant(vuln_type)
            } else {
                true
            };
            
            severity_match && type_match
        })
        .cloned()
        .collect();
    
    if filtered_findings.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            ui.label(egui::RichText::new("🎉 No vulnerabilities found!")
                .size(20.0)
                .strong()
                .color(egui::Color32::GREEN));
            ui.label("Your contracts appear to be secure.");
        });
    } else {
        // Show active filters
        let mut active_filters = Vec::new();
        if let Some(severity) = &state.selected_severity {
            active_filters.push(format!("Severity: {:?}", severity));
        }
        if let Some(vuln_type) = &state.selected_vulnerability_type {
            active_filters.push(format!("Type: {:?}", vuln_type));
        }
        
        if !active_filters.is_empty() {
            ui.horizontal(|ui| {
                ui.label("Active filters:");
                for filter in &active_filters {
                    ui.label(egui::RichText::new(filter)
                        .color(egui::Color32::from_rgb(59, 130, 246))
                        .background_color(egui::Color32::from_rgb(59, 130, 246).linear_multiply(0.1)));
                }
            });
            ui.add_space(5.0);
        }
        
        // Export buttons
        ui.horizontal(|ui| {
            ui.label("Export:");
            if ui.button("📤 TXT").clicked() {
                export_scan_report(state, &filtered_findings, "txt");
            }
            if ui.button("📋 JSON").clicked() {
                export_scan_report(state, &filtered_findings, "json");
            }
            if ui.button("📊 CSV").clicked() {
                export_scan_report(state, &filtered_findings, "csv");
            }
            if ui.button("📊 Summary").clicked() {
                export_summary_report(state);
            }
        });
        
        ui.add_space(5.0);
        
        ui.label(format!("Found {} vulnerability(ies):", filtered_findings.len()));
        ui.add_space(10.0);
        
        egui::ScrollArea::vertical().id_source("scan_results_scroll").max_height(500.0).show(ui, |ui| {
            for finding in &filtered_findings {
                render_vulnerability_finding(ui, finding, state);
            }
        });
    }
}

fn render_vulnerability_finding(ui: &mut egui::Ui, finding: &crate::models::VulnerabilityFinding, state: &mut AppState) {
    ui.group(|ui| {
        // Severity indicator and title
        ui.horizontal(|ui| {
            let (severity_color, severity_icon) = match finding.severity {
                Severity::Critical => (egui::Color32::RED, "🔴"),
                Severity::High => (egui::Color32::from_rgb(245, 158, 11), "🟡"),
                Severity::Medium => (egui::Color32::YELLOW, "🟠"),
                Severity::Low => (egui::Color32::GREEN, "🟢"),
                Severity::Info => (egui::Color32::BLUE, "🔵"),
            };
            
            ui.label(severity_icon);
            ui.label(egui::RichText::new(&finding.warning)
                .color(severity_color)
                .strong());
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("Line {}", finding.line));
            });
        });
        
        ui.add_space(5.0);
        
        // Code snippet
        ui.group(|ui| {
            ui.label(egui::RichText::new("Code:")
                .size(12.0)
                .strong());
            ui.label(egui::RichText::new(&finding.snippet)
                .monospace()
                .background_color(egui::Color32::from_gray(20)));
        });
        
        ui.add_space(5.0);
        
        // Explanation and fix
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Explanation:")
                    .size(12.0)
                    .strong());
                ui.label(&finding.explanation);
            });
            
            ui.add_space(20.0);
            
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Fix Suggestion:")
                    .size(12.0)
                    .strong());
                ui.label(&finding.fix_suggestion);
            });
        });
        
        // Real-world example
        if let Some(example) = &finding.real_world_example {
            ui.add_space(5.0);
            ui.label(egui::RichText::new("Real-world Example:")
                .size(12.0)
                .strong()
                .color(egui::Color32::from_rgb(239, 68, 68)));
            ui.label(example);
        }
        
        // Expandable details
        if ui.button("ℹ️ Show More Details").clicked() {
            state.show_educational_content = !state.show_educational_content;
        }
        
        if state.show_educational_content {
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            
            ui.label(egui::RichText::new("Detailed Analysis:")
                .size(12.0)
                .strong());
            ui.label("This vulnerability could lead to serious security issues. Consider implementing the suggested fix immediately.");
        }
    });
    
    ui.add_space(10.0);
}

fn scan_all_contracts(state: &mut AppState) {
    state.scan_in_progress = true;
    let mut all_findings = Vec::new();
    let mut contracts_scanned = Vec::new();
    
    for contract in &state.contracts {
        if let Ok((findings, _)) = services::scan_contract_with_timing(&contract.path) {
            let mut findings_with_contract = findings;
            // Add contract name to findings
            for finding in &mut findings_with_contract {
                finding.snippet = format!("[{}] {}", contract.name, finding.snippet);
            }
            all_findings.extend(findings_with_contract);
            contracts_scanned.push(contract.name.clone());
        }
    }
    
    state.vulnerability_findings = all_findings;
    state.scan_in_progress = false;
    
    // Update summary
    let summary = &mut state.scan_results_summary;
    summary.total_contracts = contracts_scanned.len();
    summary.total_findings = state.vulnerability_findings.len();
    summary.critical_findings = state.vulnerability_findings.iter()
        .filter(|f| matches!(f.severity, Severity::Critical))
        .count();
    summary.high_findings = state.vulnerability_findings.iter()
        .filter(|f| matches!(f.severity, Severity::High))
        .count();
    summary.medium_findings = state.vulnerability_findings.iter()
        .filter(|f| matches!(f.severity, Severity::Medium))
        .count();
    summary.low_findings = state.vulnerability_findings.iter()
        .filter(|f| matches!(f.severity, Severity::Low))
        .count();
    summary.contracts_scanned = contracts_scanned;
    
    state.success_message = Some("Scan completed successfully!".to_string());
}

fn export_scan_report(state: &mut AppState, findings: &[crate::models::VulnerabilityFinding], format: &str) {
    let (report, extension) = match format {
        "json" => (crate::services::generate_json_report(findings), "json"),
        "csv" => (crate::services::generate_csv_report(findings), "csv"),
        _ => (crate::services::generate_report(findings), "txt"),
    };
    
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("security_report_{}.{}", timestamp, extension);

    match utils::export_report(&report, &filename) {
        Ok(_) => {
            state.success_message = Some(format!("{} report exported successfully to Documents/Olympix Pattern Tool/{}", 
                format.to_uppercase(), filename));
        }
        Err(e) => {
            state.error_message = Some(format!("Failed to save {} report: {}", format.to_uppercase(), e));
        }
    }
}

fn export_summary_report(state: &mut AppState) {
    let summary = &state.scan_results_summary;
    let report = format!(
        "Scan Summary Report\nGenerated: {}\n\nTotal Contracts Scanned: {}\nTotal Vulnerabilities Found: {}\n\nSeverity Breakdown:\n- Critical: {}\n- High: {}\n- Medium: {}\n- Low: {}\n\nScan Duration: {}\n\nContracts Scanned:\n{}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        summary.total_contracts,
        summary.total_findings,
        summary.critical_findings,
        summary.high_findings,
        summary.medium_findings,
        summary.low_findings,
        state.last_scan_time.map_or("N/A".to_string(), |t| format!("{:.2} seconds", t)),
        summary.contracts_scanned.join("\n")
    );

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("scan_summary_{}.txt", timestamp);

    match utils::export_report(&report, &filename) {
        Ok(_) => {
            state.success_message = Some(format!("Scan summary exported successfully to Documents/Olympix Pattern Tool/{}", filename));
        }
        Err(e) => {
            state.error_message = Some(format!("Failed to save scan summary: {}", e));
        }
    }
} 