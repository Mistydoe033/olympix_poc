use eframe::egui;
use crate::models::{AppState, Tab, View};
use crate::utils;
use crate::gui::views::{render_contracts_tab, render_patterns_tab, render_scan_tab, render_examples_tab, render_contract_detail, render_analysis_result};

pub struct App {
    state: AppState,
}

impl Default for App {
    fn default() -> Self {
        // Ensure directories exist
        if let Err(e) = utils::ensure_directories_exist() {
            eprintln!("Failed to create directories: {}", e);
        }
        
        let contracts = utils::list_contracts();
        let patterns = utils::list_patterns();
        let state = AppState {
            current_tab: Tab::Contracts,
            current_view: View::Home,
            contracts,
            patterns,
            selected_contract: None,
            selected_pattern: None,
            contract_source: None,
            vulnerability_findings: Vec::new(),
            selected_severity: None,
            show_educational_content: false,
            scan_in_progress: false,
            last_scan_time: None,
            error_message: None,
            success_message: None,
            import_dialog_open: false,
            import_file_type: None,
            scan_results_summary: Default::default(),
        };
        
        Self { state }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Clear messages after a delay
        if let Some(_) = self.state.error_message {
            if ctx.input(|i| i.time) > 5.0 {
                self.state.error_message = None;
            }
        }
        if let Some(_) = self.state.success_message {
            if ctx.input(|i| i.time) > 3.0 {
                self.state.success_message = None;
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            render_header(ui, &mut self.state);
            
            // Tab bar
            render_tab_bar(ui, &mut self.state);
            
            ui.add_space(10.0);
            
            // Tab content with view switching
            match self.state.current_tab {
                Tab::Contracts => {
                    match self.state.current_view {
                        View::Home => render_contracts_tab(ui, &mut self.state),
                        View::ContractDetail => render_contract_detail(ui, &mut self.state),
                        View::AnalysisResult => render_analysis_result(ui, &mut self.state),
                        _ => render_contracts_tab(ui, &mut self.state),
                    }
                },
                Tab::Patterns => render_patterns_tab(ui, &mut self.state),
                Tab::Scan => render_scan_tab(ui, &mut self.state),
                Tab::Examples => render_examples_tab(ui, &mut self.state),
            }
        });
    }
}

fn render_header(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.heading(egui::RichText::new("🔒 Olympix Pattern Tool")
            .size(24.0)
            .strong()
            .color(egui::Color32::from_rgb(59, 130, 246)));
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if let Some(error) = &state.error_message {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
            }
            if let Some(success) = &state.success_message {
                ui.colored_label(egui::Color32::GREEN, format!("✅ {}", success));
            }
        });
    });
    
    ui.label(egui::RichText::new("Professional Smart Contract Security Scanner")
        .size(14.0)
        .color(egui::Color32::GRAY));
}

fn render_tab_bar(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.current_tab, Tab::Contracts, 
            egui::RichText::new("📄 Contracts").size(16.0));
        ui.selectable_value(&mut state.current_tab, Tab::Patterns, 
            egui::RichText::new("🔍 Patterns").size(16.0));
        ui.selectable_value(&mut state.current_tab, Tab::Scan, 
            egui::RichText::new("🚀 Scan").size(16.0));
        ui.selectable_value(&mut state.current_tab, Tab::Examples, 
            egui::RichText::new("💡 Examples").size(16.0));
    });
    
    ui.add_space(5.0);
    ui.separator();
} 