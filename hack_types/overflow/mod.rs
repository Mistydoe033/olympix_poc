// Overflow hack simulation module

use revm::{Evm, Env, Bytecode, TransactTo};

pub struct OverflowFinding {
    pub line: usize,
    pub snippet: String,
    pub warning: String,
}

pub fn scan_for_overflow_patterns(source: &str) -> Vec<OverflowFinding> {
    let mut findings = Vec::new();
    let mut in_block_comment = false;
    let mut in_unchecked_block = false;
    for (i, line) in source.lines().enumerate() {
        let mut l = line.trim();
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
        // Detect start/end of unchecked blocks
        if l.contains("unchecked {") {
            in_unchecked_block = true;
        }
        if in_unchecked_block {
            if l.contains("}") {
                in_unchecked_block = false;
            }
            continue;
        }
        // Remove inline comments
        if let Some(idx) = l.find("//") {
            l = &l[..idx];
        }
        // Look for arithmetic operations without SafeMath or unchecked
        if (l.contains("+") || l.contains("-") || l.contains("*"))
            && !l.contains("SafeMath")
        {
            findings.push(OverflowFinding {
                line: i + 1,
                snippet: l.to_string(),
                warning: "Potential overflow-prone arithmetic operation".to_string(),
            });
        }
    }
    findings
}

pub fn simulate_overflow_execution(bytecode_hex: &str) -> String {
    // Convert hex string to bytes
    let bytecode_bytes = match hex::decode(bytecode_hex) {
        Ok(bytes) => bytes,
        Err(_) => return "Invalid bytecode hex".to_string(),
    };
    let mut evm = Evm::new();
    let mut env = Env::default();
    // Deploy contract
    let bytecode = Bytecode::new_raw(bytecode_bytes.into());
    let deploy_result = evm.transact_create(bytecode, 0, None);
    match deploy_result {
        Ok(res) => {
            if res.exit_reason.is_succeed() {
                // Optionally, simulate a call to a function that could overflow
                // For now, just report success
                format!("Contract deployed. Address: {:?}", res.address)
            } else {
                format!("Deployment failed: {:?}", res.exit_reason)
            }
        }
        Err(e) => format!("EVM error: {:?}", e),
    }
} 