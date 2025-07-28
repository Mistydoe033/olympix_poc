use std::path::PathBuf;

// Main application tabs
#[derive(Clone, PartialEq)]
pub enum Tab {
    Contracts,
    Patterns,
    Scan,
    Examples,
}

// Vulnerability detection structures
#[derive(Clone)]
pub struct VulnerabilityFinding {
    pub line: usize,
    pub snippet: String,
    pub warning: String,
    pub severity: Severity,
    pub category: VulnerabilityType,
    pub explanation: String,
    pub fix_suggestion: String,
    pub real_world_example: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Clone, PartialEq)]
pub enum VulnerabilityType {
    Overflow,
    Reentrancy,
    AccessControl,
    UncheckedExternalCall,
    TimestampDependence,
    GasLimit,
    UncheckedReturn,
    FrontRunning,
    FlashLoan,
    OracleManipulation,
}

#[derive(Clone)]
pub enum Action {
    SelectContract(usize, PathBuf),
    ScanContract(usize),
    ImportFile(PathBuf, FileType),
    ImportFolder(PathBuf, FileType),
}

#[derive(Clone, PartialEq)]
pub enum FileType {
    Contract,
    RustPattern,
    RegexPattern,
}

#[derive(Clone)]
pub struct ContractInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub last_modified: std::time::SystemTime,
}

#[derive(Clone)]
pub struct PatternInfo {
    pub name: String,
    pub path: PathBuf,
    pub pattern_type: FileType,
    pub description: String,
    pub size: u64,
    pub last_modified: std::time::SystemTime,
}

#[derive(Clone)]
pub enum View {
    Home,
    ContractDetail,
    AnalysisResult,
    Educational,
}

pub struct AppState {
    pub current_tab: Tab,
    pub current_view: View,
    pub contracts: Vec<ContractInfo>,
    pub patterns: Vec<PatternInfo>,
    pub selected_contract: Option<usize>,
    pub selected_pattern: Option<usize>,
    pub contract_source: Option<String>,
    pub pattern_source: Option<String>,
    pub vulnerability_findings: Vec<VulnerabilityFinding>,
    pub selected_severity: Option<Severity>,
    pub show_educational_content: bool,
    pub scan_in_progress: bool,
    pub last_scan_time: Option<f64>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub import_dialog_open: bool,
    pub import_file_type: Option<FileType>,
    pub scan_results_summary: ScanSummary,
    pub show_pattern_view: bool,
}

#[derive(Clone)]
pub struct ScanSummary {
    pub total_contracts: usize,
    pub total_findings: usize,
    pub critical_findings: usize,
    pub high_findings: usize,
    pub medium_findings: usize,
    pub low_findings: usize,
    pub scan_duration: f64,
    pub contracts_scanned: Vec<String>,
}

impl Default for ScanSummary {
    fn default() -> Self {
        Self {
            total_contracts: 0,
            total_findings: 0,
            critical_findings: 0,
            high_findings: 0,
            medium_findings: 0,
            low_findings: 0,
            scan_duration: 0.0,
            contracts_scanned: Vec::new(),
        }
    }
}

// Real-world exploit examples
#[derive(Clone)]
pub struct ExploitExample {
    pub name: String,
    pub description: String,
    pub date: String,
    pub loss_amount: String,
    pub vulnerability_type: VulnerabilityType,
    pub vulnerable_code: String,
    pub fixed_code: String,
    pub how_our_tool_detects: String,
    pub prevention_tip: String,
} 