// Rust pattern for detecting integer overflow vulnerabilities
// This pattern can be used to analyze Solidity contracts for overflow issues

use regex::Regex;
use std::collections::HashMap;

pub struct OverflowDetector {
    patterns: Vec<Regex>,
    results: HashMap<String, Vec<Match>>,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub line_number: usize,
    pub matched_text: String,
    pub pattern_name: String,
    pub severity: Severity,
}

#[derive(Debug, Clone)]
pub enum Severity {
    High,
    Medium,
    Low,
}

impl OverflowDetector {
    pub fn new() -> Self {
        let patterns = vec![
            // Basic arithmetic operations
            Regex::new(r"(\w+)\s*\+\s*(\w+)").unwrap(),
            Regex::new(r"(\w+)\s*-\s*(\w+)").unwrap(),
            Regex::new(r"(\w+)\s*\*\s*(\w+)").unwrap(),
            
            // Unsafe uint operations
            Regex::new(r"uint\d*\s+\w+\s*\+\s*\w+").unwrap(),
            Regex::new(r"uint\d*\s+\w+\s*-\s*\w+").unwrap(),
            
            // Loop increment operations
            Regex::new(r"for\s*\(\s*.*\+\+.*\)").unwrap(),
            Regex::new(r"while\s*\(.*\+\+.*\)").unwrap(),
        ];

        Self {
            patterns,
            results: HashMap::new(),
        }
    }

    pub fn analyze_contract(&mut self, contract_content: &str) -> HashMap<String, Vec<Match>> {
        self.results.clear();
        
        for (pattern_index, pattern) in self.patterns.iter().enumerate() {
            let pattern_name = match pattern_index {
                0 => "addition_operation",
                1 => "subtraction_operation", 
                2 => "multiplication_operation",
                3 => "uint_addition",
                4 => "uint_subtraction",
                5 => "for_loop_increment",
                6 => "while_loop_increment",
                _ => "unknown_pattern",
            };

            let mut matches = Vec::new();
            
            for (line_index, line) in contract_content.lines().enumerate() {
                for capture in pattern.find_iter(line) {
                    let severity = match pattern_index {
                        0..=2 => Severity::Medium,
                        3..=4 => Severity::High,
                        5..=6 => Severity::Low,
                        _ => Severity::Low,
                    };

                    matches.push(Match {
                        line_number: line_index + 1,
                        matched_text: capture.as_str().to_string(),
                        pattern_name: pattern_name.to_string(),
                        severity,
                    });
                }
            }

            if !matches.is_empty() {
                self.results.insert(pattern_name.to_string(), matches);
            }
        }

        self.results.clone()
    }

    pub fn get_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Overflow Detection Summary:\n");
        summary.push_str("==========================\n\n");

        for (pattern_name, matches) in &self.results {
            summary.push_str(&format!("{}: {} matches\n", pattern_name, matches.len()));
            
            for m in matches {
                summary.push_str(&format!("  Line {}: {}\n", m.line_number, m.matched_text));
            }
            summary.push_str("\n");
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_detection() {
        let mut detector = OverflowDetector::new();
        
        let test_contract = r#"
            contract Test {
                uint256 public balance;
                
                function add(uint256 a, uint256 b) public {
                    balance = balance + a;  // Potential overflow
                    uint256 result = a + b; // Potential overflow
                }
                
                function loop() public {
                    for (uint i = 0; i < 10; i++) {
                        balance = balance + 1;
                    }
                }
            }
        "#;

        let results = detector.analyze_contract(test_contract);
        assert!(!results.is_empty());
    }
} 