# Olympix Pattern Tool - Solidity Security Scanner

A cross-platform application for detecting vulnerabilities in Solidity smart contracts.

## 🚀 How to Run (Developer)

### Prerequisites

- **Rust** (latest stable version)

### Installing Rust on Mac

**Option 1: Using rustup (Recommended)**

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts and restart your terminal
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

**Option 2: Using Homebrew**

```bash
# Install Homebrew if you don't have it
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Rust
brew install rust

# Verify installation
rustc --version
cargo --version
```

### Quick Start

1. **Clone and navigate:**

   ```bash
   git clone https://github.com/Mistydoe033/olympix_poc.git
   cd olympix_poc
   ```

2. **Run with Cargo:**

   **Mac/Linux:**

   ```bash
   cargo run
   ```

   **Windows:**

   ```cmd
   cargo run
   ```

3. **Or install as native app:**
   - Use the platform-specific installer
   - Copy the `contracts` folder contents to the application's contracts directory

### What You'll See

- Native GUI application launches
- Navigate to the **Contracts** tab and find `OverflowVulnerability.sol`, then click scan
- Automatically scans sample contracts in the `contracts/` folder

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
