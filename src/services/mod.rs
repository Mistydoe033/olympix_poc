use std::time::Instant;
use crate::models::{VulnerabilityFinding, Severity, VulnerabilityType, ExploitExample};
use crate::utils;

pub fn scan_for_vulnerabilities(source: &str) -> Vec<VulnerabilityFinding> {
    let mut findings = Vec::new();
    let mut in_block_comment = false;
    let mut in_unchecked_block = false;
    
    for (i, line) in source.lines().enumerate() {
        let l = line.trim();
        
        // Handle block comments
        if l.contains("/*") {
            in_block_comment = true;
        }
        if in_block_comment {
            if l.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }
        
        // Ignore single-line comments
        if l.starts_with("//") || l.is_empty() {
            continue;
        }
        
        // Detect unchecked blocks
        if l.contains("unchecked") {
            in_unchecked_block = true;
        }
        if in_unchecked_block && l.contains("}") {
            in_unchecked_block = false;
        }
        
        // Check for vulnerabilities
        check_overflow_underflow(&mut findings, i, l, in_unchecked_block);
        check_reentrancy(&mut findings, i, l);
        check_access_control(&mut findings, i, l);
        check_unchecked_external_calls(&mut findings, i, l);
        check_timestamp_dependence(&mut findings, i, l);
        check_gas_limit_issues(&mut findings, i, l);
        check_unchecked_return_values(&mut findings, i, l);
        check_front_running(&mut findings, i, l);
        check_flash_loan_attacks(&mut findings, i, l);
        check_oracle_manipulation(&mut findings, i, l);
    }
    
    findings
}

fn check_overflow_underflow(
    findings: &mut Vec<VulnerabilityFinding>, 
    line_num: usize, 
    line: &str, 
    in_unchecked_block: bool
) {
    if !in_unchecked_block {
        if line.contains("+") || line.contains("-") || line.contains("*") {
            if !line.contains("SafeMath") && !line.contains("checked") {
                findings.push(VulnerabilityFinding {
                    line: line_num + 1,
                    snippet: line.to_string(),
                    warning: "Potential arithmetic overflow/underflow".to_string(),
                    severity: Severity::Critical,
                    category: VulnerabilityType::Overflow,
                    explanation: "Arithmetic operations without overflow checks can lead to unexpected behavior and potential exploits.".to_string(),
                    fix_suggestion: "Use SafeMath library or Solidity 0.8+ built-in overflow protection.".to_string(),
                    real_world_example: Some("The DAO hack (2016) - $60M lost due to integer overflow".to_string()),
                });
            }
        }
    }
}

fn check_reentrancy(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("call") || line.contains("send") || line.contains("transfer") {
        if !line.contains("reentrancy") && !line.contains("nonReentrant") {
            findings.push(VulnerabilityFinding {
                line: line_num + 1,
                snippet: line.to_string(),
                warning: "Potential reentrancy vulnerability".to_string(),
                severity: Severity::Critical,
                category: VulnerabilityType::Reentrancy,
                explanation: "External calls can allow attackers to re-enter the contract before state changes are applied.".to_string(),
                fix_suggestion: "Use reentrancy guards, follow checks-effects-interactions pattern, or use pull payment pattern.".to_string(),
                real_world_example: Some("The DAO hack (2016) - $60M stolen through reentrancy attack".to_string()),
            });
        }
    }
}

fn check_access_control(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("function") && !line.contains("public") && !line.contains("external") {
        if !line.contains("private") && !line.contains("internal") {
            findings.push(VulnerabilityFinding {
                line: line_num + 1,
                snippet: line.to_string(),
                warning: "Missing access control modifier".to_string(),
                severity: Severity::High,
                category: VulnerabilityType::AccessControl,
                explanation: "Functions without proper access control can be called by unauthorized users.".to_string(),
                fix_suggestion: "Add appropriate modifiers like onlyOwner, onlyRole, or custom access control.".to_string(),
                real_world_example: Some("Parity Wallet hack (2017) - $30M lost due to missing access control".to_string()),
            });
        }
    }
}

fn check_unchecked_external_calls(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("call(") || line.contains("delegatecall(") || line.contains("staticcall(") {
        if !line.contains("require") && !line.contains("assert") {
            findings.push(VulnerabilityFinding {
                line: line_num + 1,
                snippet: line.to_string(),
                warning: "Unchecked external call".to_string(),
                severity: Severity::High,
                category: VulnerabilityType::UncheckedExternalCall,
                explanation: "Low-level calls can fail silently, leading to unexpected behavior.".to_string(),
                fix_suggestion: "Always check the return value of external calls and handle failures appropriately.".to_string(),
                real_world_example: Some("King of the Ether (2016) - Unchecked external calls led to contract failures".to_string()),
            });
        }
    }
}

fn check_timestamp_dependence(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("block.timestamp") {
        findings.push(VulnerabilityFinding {
            line: line_num + 1,
            snippet: line.to_string(),
            warning: "Timestamp dependence detected".to_string(),
            severity: Severity::Medium,
            category: VulnerabilityType::TimestampDependence,
            explanation: "Block timestamps can be manipulated by miners within a small range.".to_string(),
            fix_suggestion: "Use block numbers or implement time-based logic with appropriate tolerances.".to_string(),
            real_world_example: Some("Various DEX exploits - Timestamp manipulation for MEV attacks".to_string()),
        });
    }
}

fn check_gas_limit_issues(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("for") || line.contains("while") {
        if line.contains("call") || line.contains("transfer") {
            findings.push(VulnerabilityFinding {
                line: line_num + 1,
                snippet: line.to_string(),
                warning: "Loop with external calls may hit gas limit".to_string(),
                severity: Severity::Medium,
                category: VulnerabilityType::GasLimit,
                explanation: "Loops with external calls can consume unpredictable amounts of gas.".to_string(),
                fix_suggestion: "Use pull payment pattern or limit loop iterations to prevent gas limit issues.".to_string(),
                real_world_example: Some("Governor Bravo (2020) - Gas limit issues in governance proposals".to_string()),
            });
        }
    }
}

fn check_unchecked_return_values(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("transfer(") || line.contains("send(") {
        if !line.contains("require") && !line.contains("assert") {
            findings.push(VulnerabilityFinding {
                line: line_num + 1,
                snippet: line.to_string(),
                warning: "Unchecked return value from transfer/send".to_string(),
                severity: Severity::Medium,
                category: VulnerabilityType::UncheckedReturn,
                explanation: "transfer() and send() can fail, but the return value is not being checked.".to_string(),
                fix_suggestion: "Use call() with proper return value checking, or use require() to check transfer success.".to_string(),
                real_world_example: Some("Various DeFi protocols - Failed transfers not properly handled".to_string()),
            });
        }
    }
}

fn check_front_running(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("swap") || line.contains("trade") || line.contains("order") {
        if line.contains("public") && !line.contains("private") {
            findings.push(VulnerabilityFinding {
                line: line_num + 1,
                snippet: line.to_string(),
                warning: "Potential front-running vulnerability".to_string(),
                severity: Severity::High,
                category: VulnerabilityType::FrontRunning,
                explanation: "Public trading functions can be front-run by MEV bots.".to_string(),
                fix_suggestion: "Use commit-reveal schemes, private mempools, or time-delayed execution.".to_string(),
                real_world_example: Some("Uniswap V2 (2020) - MEV bots front-running trades for profit".to_string()),
            });
        }
    }
}

fn check_flash_loan_attacks(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("flash") || line.contains("borrow") {
        if !line.contains("require") && !line.contains("check") {
            findings.push(VulnerabilityFinding {
                line: line_num + 1,
                snippet: line.to_string(),
                warning: "Potential flash loan attack vulnerability".to_string(),
                severity: Severity::Critical,
                category: VulnerabilityType::FlashLoan,
                explanation: "Flash loans can be used to manipulate prices and drain funds.".to_string(),
                fix_suggestion: "Implement proper checks and use oracle price feeds with manipulation protection.".to_string(),
                real_world_example: Some("Harvest Finance (2020) - $24M stolen through flash loan attack".to_string()),
            });
        }
    }
}

fn check_oracle_manipulation(findings: &mut Vec<VulnerabilityFinding>, line_num: usize, line: &str) {
    if line.contains("price") || line.contains("oracle") || line.contains("feed") {
        if line.contains("single") || !line.contains("multiple") {
            findings.push(VulnerabilityFinding {
                line: line_num + 1,
                snippet: line.to_string(),
                warning: "Single oracle dependency detected".to_string(),
                severity: Severity::High,
                category: VulnerabilityType::OracleManipulation,
                explanation: "Single oracle sources can be manipulated or become stale.".to_string(),
                fix_suggestion: "Use multiple oracle sources, implement circuit breakers, and add manipulation detection.".to_string(),
                real_world_example: Some("Synthetix (2019) - Oracle manipulation led to incorrect price feeds".to_string()),
            });
        }
    }
}

pub fn scan_contract_with_timing(contract_path: &std::path::Path) -> Result<(Vec<VulnerabilityFinding>, f64), std::io::Error> {
    let start_time = Instant::now();
    let source = utils::read_file_content(contract_path)?;
    let findings = scan_for_vulnerabilities(&source);
    let scan_time = start_time.elapsed().as_secs_f64();
    Ok((findings, scan_time))
}

pub fn generate_report(findings: &[VulnerabilityFinding]) -> String {
    let mut report = String::new();
    report.push_str("🔒 Solidity Security Scanner Report\n");
    report.push_str("================================\n\n");
    
    report.push_str(&format!("Total Findings: {}\n", findings.len()));
    
    let critical_count = findings.iter()
        .filter(|f| matches!(f.severity, Severity::Critical))
        .count();
    let high_count = findings.iter()
        .filter(|f| matches!(f.severity, Severity::High))
        .count();
    let medium_count = findings.iter()
        .filter(|f| matches!(f.severity, Severity::Medium))
        .count();
    let low_count = findings.iter()
        .filter(|f| matches!(f.severity, Severity::Low))
        .count();
    
    report.push_str(&format!("Critical Severity: {}\n", critical_count));
    report.push_str(&format!("High Severity: {}\n", high_count));
    report.push_str(&format!("Medium Severity: {}\n", medium_count));
    report.push_str(&format!("Low Severity: {}\n\n", low_count));
    
    for finding in findings {
        report.push_str(&format!("[{}] Line {}: {}\n", 
            match finding.severity {
                Severity::Critical => "CRITICAL",
                Severity::High => "HIGH",
                Severity::Medium => "MEDIUM", 
                Severity::Low => "LOW",
                Severity::Info => "INFO",
            },
            finding.line,
            finding.warning
        ));
        report.push_str(&format!("Code: {}\n", finding.snippet));
        report.push_str(&format!("Explanation: {}\n", finding.explanation));
        report.push_str(&format!("Fix: {}\n", finding.fix_suggestion));
        if let Some(example) = &finding.real_world_example {
            report.push_str(&format!("Real-world example: {}\n", example));
        }
        report.push_str("\n");
    }
    
    report
}

pub fn generate_json_report(findings: &[VulnerabilityFinding]) -> String {
    use serde_json;
    
    #[derive(serde::Serialize)]
    struct FindingJson {
        severity: String,
        category: String,
        line: usize,
        warning: String,
        snippet: String,
        explanation: String,
        fix_suggestion: String,
        real_world_example: Option<String>,
    }
    
    #[derive(serde::Serialize)]
    struct ReportJson {
        timestamp: String,
        total_findings: usize,
        findings: Vec<FindingJson>,
        summary: SummaryJson,
    }
    
    #[derive(serde::Serialize)]
    struct SummaryJson {
        critical: usize,
        high: usize,
        medium: usize,
        low: usize,
    }
    
    let findings_json: Vec<FindingJson> = findings.iter().map(|f| FindingJson {
        severity: match f.severity {
            Severity::Critical => "Critical".to_string(),
            Severity::High => "High".to_string(),
            Severity::Medium => "Medium".to_string(),
            Severity::Low => "Low".to_string(),
            Severity::Info => "Info".to_string(),
        },
        category: format!("{:?}", f.category),
        line: f.line,
        warning: f.warning.clone(),
        snippet: f.snippet.clone(),
        explanation: f.explanation.clone(),
        fix_suggestion: f.fix_suggestion.clone(),
        real_world_example: f.real_world_example.clone(),
    }).collect();
    
    let summary = SummaryJson {
        critical: findings.iter().filter(|f| matches!(f.severity, Severity::Critical)).count(),
        high: findings.iter().filter(|f| matches!(f.severity, Severity::High)).count(),
        medium: findings.iter().filter(|f| matches!(f.severity, Severity::Medium)).count(),
        low: findings.iter().filter(|f| matches!(f.severity, Severity::Low)).count(),
    };
    
    let report = ReportJson {
        timestamp: chrono::Utc::now().to_rfc3339(),
        total_findings: findings.len(),
        findings: findings_json,
        summary,
    };
    
    serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
}

pub fn generate_csv_report(findings: &[VulnerabilityFinding]) -> String {
    let mut csv = String::new();
    csv.push_str("Severity,Category,Line,Warning,Code,Explanation,Fix Suggestion,Real World Example\n");
    
    for finding in findings {
        let severity = match finding.severity {
            Severity::Critical => "Critical",
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
            Severity::Info => "Info",
        };
        
        let category = format!("{:?}", finding.category);
        let real_world_example = finding.real_world_example.as_deref().unwrap_or("");
        
        // Escape CSV fields that contain commas or quotes
        let escape_csv = |s: &str| {
            if s.contains(',') || s.contains('"') || s.contains('\n') {
                format!("\"{}\"", s.replace("\"", "\"\""))
            } else {
                s.to_string()
            }
        };
        
        csv.push_str(&format!("{},{},{},{},{},{},{},{}\n",
            severity,
            category,
            finding.line,
            escape_csv(&finding.warning),
            escape_csv(&finding.snippet),
            escape_csv(&finding.explanation),
            escape_csv(&finding.fix_suggestion),
            escape_csv(real_world_example)
        ));
    }
    
    csv
}

pub fn get_real_world_examples() -> Vec<ExploitExample> {
    vec![
        ExploitExample {
            name: "The DAO Hack (2016)".to_string(),
            description: "The most famous smart contract exploit in history".to_string(),
            date: "June 2016".to_string(),
            loss_amount: "$60 Million".to_string(),
            vulnerability_type: VulnerabilityType::Reentrancy,
            vulnerable_code: "function withdraw(uint amount) {\n    if (balances[msg.sender] >= amount) {\n        msg.sender.call.value(amount)();\n        balances[msg.sender] -= amount;\n    }\n}".to_string(),
            fixed_code: "function withdraw(uint amount) {\n    require(balances[msg.sender] >= amount);\n    balances[msg.sender] -= amount;\n    msg.sender.call.value(amount)();\n}".to_string(),
            how_our_tool_detects: "Detects external calls before state changes and suggests reentrancy guards".to_string(),
            prevention_tip: "Always follow the checks-effects-interactions pattern".to_string(),
        },
        ExploitExample {
            name: "Parity Wallet Hack (2017)".to_string(),
            description: "Multi-signature wallet vulnerability".to_string(),
            date: "July 2017".to_string(),
            loss_amount: "$30 Million".to_string(),
            vulnerability_type: VulnerabilityType::AccessControl,
            vulnerable_code: "function initWallet(address[] _owners, uint _required) {\n    // No access control\n    owners = _owners;\n    required = _required;\n}".to_string(),
            fixed_code: "function initWallet(address[] _owners, uint _required) {\n    require(msg.sender == owner);\n    require(!initialized);\n    owners = _owners;\n    required = _required;\n    initialized = true;\n}".to_string(),
            how_our_tool_detects: "Identifies functions without proper access control modifiers".to_string(),
            prevention_tip: "Always add access control to critical functions".to_string(),
        },
        ExploitExample {
            name: "Harvest Finance Flash Loan (2020)".to_string(),
            description: "Flash loan attack on DeFi protocol".to_string(),
            date: "October 2020".to_string(),
            loss_amount: "$24 Million".to_string(),
            vulnerability_type: VulnerabilityType::FlashLoan,
            vulnerable_code: "function deposit() external {\n    uint256 amount = IERC20(token).balanceOf(address(this));\n    // No flash loan protection\n    _mint(msg.sender, amount);\n}".to_string(),
            fixed_code: "function deposit() external {\n    uint256 amount = IERC20(token).balanceOf(address(this));\n    require(amount > 0, 'No tokens received');\n    require(!_isFlashLoan(), 'Flash loans not allowed');\n    _mint(msg.sender, amount);\n}".to_string(),
            how_our_tool_detects: "Detects flash loan patterns and suggests protection mechanisms".to_string(),
            prevention_tip: "Implement flash loan detection and use oracle price feeds".to_string(),
        },
    ]
} 