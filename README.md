# Olympix Pattern Tool - Solidity Security Scanner

A Windows-native application for detecting vulnerabilities in Solidity smart contracts.

## 🚀 How to Run (Developer)

### Prerequisites

- **Rust** (latest stable version)
- **Windows** (native application)

### Quick Start

1. **Clone and navigate:**

   ```bash
   git clone https://github.com/Mistydoe033/olympix_poc.git
   cd olympix_poc
   ```

2. **Run with Cargo:**

   ```bash
   cargo run
   ```

3. **Or install as Windows app:**
   - Use the `olympix_pattern_tool-installer.exe`
   - Copy the `contracts` folder contents to `Documents/Olympix Pattern Tool/contracts`

### What You'll See

- Native Windows GUI application launches
- Navigate to the **Contracts** tab and find `OverflowVulnerability.sol`, then click scan

### Sample Results

In the **Scan** tab, use "Scan all contracts" to detect 10 vulnerability types:

- **Overflow** (like Cetus Protocol hack) - _focused on this_
- **Reentrancy** attacks
- **Access Control** issues
- And 7 other common smart contract vulnerabilities

### Patterns Folder

The `patterns/` folder is a POC for advanced vulnerability detection.

### Demo

See `Demo.md` for detailed explanation of how the overflow detection works.
