pub mod contracts_tab;
pub mod patterns_tab;
pub mod scan_tab;
pub mod examples_tab;
pub mod contract_detail;
pub mod analysis_result;

pub use contracts_tab::render_contracts_tab;
pub use patterns_tab::render_patterns_tab;
pub use scan_tab::render_scan_tab;
pub use examples_tab::render_examples_tab;
pub use contract_detail::render_contract_detail;
pub use analysis_result::render_analysis_result; 