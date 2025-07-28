use eframe::egui;
use crate::models::{AppState, View, Severity};
use crate::services;
use crate::utils;

pub fn render_analysis_result(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.button("← Back").clicked() {
            state.current_view = View::Home;
        }
        
        ui.heading("🔍 Security Analysis Results");
    });
    
    ui.add_space(10.0);
    
    // Show scan time if available
    if let Some(scan_time) = state.last_scan_time {
        ui.label(format!("⏱️ Scan completed in {:.2} seconds", scan_time));
    }
    
    // Severity filter
    ui.horizontal(|ui| {
        ui.label("Filter by severity:");
        if ui.button("All").clicked() {
            state.selected_severity = None;
        }
        if ui.button("🔴 Critical").clicked() {
            state.selected_severity = Some(Severity::Critical);
        }
        if ui.button("🔴 High").clicked() {
            state.selected_severity = Some(Severity::High);
        }
        if ui.button("🟡 Medium").clicked() {
            state.selected_severity = Some(Severity::Medium);
        }
        if ui.button("🟢 Low").clicked() {
            state.selected_severity = Some(Severity::Low);
        }
    });
    
    ui.add_space(10.0);
    
    // Export button
    if ui.button("📤 Export Report").clicked() {
        export_report(state);
    }
    
    ui.add_space(10.0);
    
    // Display findings
    let filtered_findings: Vec<crate::models::VulnerabilityFinding> = if let Some(severity) = &state.selected_severity {
        state.vulnerability_findings.iter()
            .filter(|f| std::mem::discriminant(&f.severity) == std::mem::discriminant(severity))
            .cloned()
            .collect()
    } else {
        state.vulnerability_findings.clone()
    };
    
    if filtered_findings.is_empty() {
        ui.label("🎉 No vulnerabilities found!");
        ui.label("Your contract appears to be secure.");
    } else {
        ui.label(format!("Found {} vulnerability(ies):", filtered_findings.len()));
        ui.add_space(10.0);
        
        for finding in &filtered_findings {
            render_finding(ui, finding, state);
        }
    }
}

fn render_finding(ui: &mut egui::Ui, finding: &crate::models::VulnerabilityFinding, state: &mut AppState) {
    ui.group(|ui| {
        let severity_color = match finding.severity {
            Severity::Critical => egui::Color32::from_rgb(139, 0, 0), // Dark red
            Severity::High => egui::Color32::RED,
            Severity::Medium => egui::Color32::YELLOW,
            Severity::Low => egui::Color32::GREEN,
            Severity::Info => egui::Color32::BLUE,
        };
        
        ui.label(egui::RichText::new(&finding.warning)
            .color(severity_color)
            .strong());
        
        ui.label(format!("Line {}: {}", finding.line, finding.snippet));
        
        if ui.button("ℹ️ Show Details").clicked() {
            state.show_educational_content = !state.show_educational_content;
        }
        
        if state.show_educational_content {
            ui.add_space(5.0);
            ui.label(egui::RichText::new("Explanation:").strong());
            ui.label(&finding.explanation);
            ui.add_space(5.0);
            ui.label(egui::RichText::new("Fix Suggestion:").strong());
            ui.label(&finding.fix_suggestion);
        }
    });
}

fn export_report(state: &mut AppState) {
    let report = services::generate_report(&state.vulnerability_findings);
    
    match utils::export_report(&report, "security_report.txt") {
        Ok(_) => {
            state.success_message = Some("Report exported successfully to Documents/Olympix Pattern Tool/security_report.txt".to_string());
        }
        Err(e) => {
            state.error_message = Some(format!("Failed to save report: {}", e));
        }
    }
} 