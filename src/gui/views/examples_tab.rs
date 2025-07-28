use eframe::egui;
use crate::models::{AppState, VulnerabilityType};
use crate::services;

pub fn render_examples_tab(ui: &mut egui::Ui, _state: &mut AppState) {
    ui.heading(egui::RichText::new("Real-World Exploit Examples")
        .size(20.0)
        .strong());
    
    ui.add_space(10.0);
    
    ui.label(egui::RichText::new("Learn from history: These are real exploits that could have been prevented with proper security scanning")
        .size(14.0)
        .color(egui::Color32::GRAY));
    
    ui.add_space(20.0);
    
    // Get real-world examples
    let examples = services::get_real_world_examples();
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        for example in examples {
            render_exploit_example(ui, &example);
            ui.add_space(20.0);
        }
    });
}

fn render_exploit_example(ui: &mut egui::Ui, example: &crate::models::ExploitExample) {
    ui.group(|ui| {
        // Header with exploit info
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(&example.name)
                    .size(18.0)
                    .strong()
                    .color(egui::Color32::RED));
                
                ui.horizontal(|ui| {
                    ui.label(format!("📅 {}", example.date));
                    ui.label("•");
                    ui.label(format!("💰 {}", example.loss_amount));
                    ui.label("•");
                    
                    let vulnerability_color = match example.vulnerability_type {
                        VulnerabilityType::Reentrancy => egui::Color32::RED,
                        VulnerabilityType::AccessControl => egui::Color32::from_rgb(245, 158, 11),
                        VulnerabilityType::FlashLoan => egui::Color32::from_rgb(239, 68, 68),
                        _ => egui::Color32::GRAY,
                    };
                    
                    ui.label(egui::RichText::new(format!("{}", get_vulnerability_name(&example.vulnerability_type)))
                        .color(vulnerability_color)
                        .strong());
                });
            });
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("🚨 CRITICAL")
                    .color(egui::Color32::RED)
                    .strong()
                    .background_color(egui::Color32::RED.linear_multiply(0.1)));
            });
        });
        
        ui.add_space(10.0);
        
        // Description
        ui.label(egui::RichText::new(&example.description)
            .size(14.0)
            .color(egui::Color32::GRAY));
        
        ui.add_space(15.0);
        
        // Code comparison
        ui.horizontal(|ui| {
            // Vulnerable code
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("❌ Vulnerable Code")
                    .size(14.0)
                    .strong()
                    .color(egui::Color32::RED));
                
                ui.group(|ui| {
                    ui.label(egui::RichText::new(&example.vulnerable_code)
                        .monospace()
                        .size(12.0)
                        .background_color(egui::Color32::RED.linear_multiply(0.1)));
                });
            });
            
            ui.add_space(20.0);
            
            // Fixed code
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("✅ Fixed Code")
                    .size(14.0)
                    .strong()
                    .color(egui::Color32::GREEN));
                
                ui.group(|ui| {
                    ui.label(egui::RichText::new(&example.fixed_code)
                        .monospace()
                        .size(12.0)
                        .background_color(egui::Color32::GREEN.linear_multiply(0.1)));
                });
            });
        });
        
        ui.add_space(15.0);
        
        // How our tool detects it
        ui.group(|ui| {
            ui.label(egui::RichText::new("🔍 How Our Tool Detects This")
                .size(14.0)
                .strong()
                .color(egui::Color32::from_rgb(59, 130, 246)));
            
            ui.add_space(5.0);
            
            ui.label(egui::RichText::new(&example.how_our_tool_detects)
                .size(12.0));
        });
        
        ui.add_space(10.0);
        
        // Prevention tip
        ui.group(|ui| {
            ui.label(egui::RichText::new("💡 Prevention Tip")
                .size(14.0)
                .strong()
                .color(egui::Color32::from_rgb(34, 197, 94)));
            
            ui.add_space(5.0);
            
            ui.label(egui::RichText::new(&example.prevention_tip)
                .size(12.0));
        });
        
        ui.add_space(10.0);
        
        // Impact assessment
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("🎯 Impact:")
                .size(12.0)
                .strong());
            ui.label("This exploit resulted in significant financial losses and could have been prevented with proper security auditing.");
        });
    });
}

fn get_vulnerability_name(vuln_type: &VulnerabilityType) -> &'static str {
    match vuln_type {
        VulnerabilityType::Reentrancy => "Reentrancy Attack",
        VulnerabilityType::AccessControl => "Access Control",
        VulnerabilityType::FlashLoan => "Flash Loan Attack",
        VulnerabilityType::Overflow => "Overflow/Underflow",
        VulnerabilityType::UncheckedExternalCall => "Unchecked External Call",
        VulnerabilityType::TimestampDependence => "Timestamp Dependence",
        VulnerabilityType::GasLimit => "Gas Limit Issue",
        VulnerabilityType::UncheckedReturn => "Unchecked Return Value",
        VulnerabilityType::FrontRunning => "Front Running",
        VulnerabilityType::OracleManipulation => "Oracle Manipulation",
    }
} 