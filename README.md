# Olympix Pattern Tool

A **standalone Windows application** for managing Solidity contracts and exploit patterns with real-time regex analysis capabilities. **No coding experience required!**

## 🚀 For End Users - Quick Start

### What You Get

- ✅ Zero coding required - Just install and run
- ✅ Standalone Windows app - No dependencies to install
- ✅ Modern native interface - User-friendly dark theme
- ✅ Fast Rust backend - Real-time pattern analysis
- ✅ Sample content included - Ready-to-use contracts and patterns
- ✅ Real-time regex testing - Test patterns on your .sol files instantly
- ✅ **Export scan results** - TXT, JSON, CSV, and summary reports

### 📦 Installation (Super Easy!)

1. **Download** the `olympix_pattern_tool-installer.exe` installer
2. **Run** the installer and follow the prompts
3. **The app will be installed to** `C:\Program Files\Olympix Pattern Tool` (or your chosen location)
4. **User data is stored in** `Documents/Olympix Pattern Tool`
5. **Shortcuts** are created in the Start Menu and optionally on the Desktop
6. **App icon** is embedded in the `.exe` and used for all shortcuts

---

## 📤 Exporting Reports

You can export scan results in multiple formats from the **Scan** tab:

- **TXT:** Human-readable report for sharing or printing
- **JSON:** For integration with other tools or scripts
- **CSV:** For spreadsheet analysis (Excel, Google Sheets, etc.)
- **Summary:** High-level scan statistics

**How to export:**

1. Run a scan
2. (Optional) Apply filters for severity/type
3. Click the export button for your desired format
4. Find the exported file in `Documents/Olympix Pattern Tool/` (timestamped)

---

## 🛠️ For Developers - Building the Installer

### Prerequisites

- Windows 10/11
- Rust (latest stable)
- Git
- Inno Setup (for building the installer)

### Build Steps

```powershell
# Run the automated build script
./build-installer.ps1
```

- The script will build the app, embed the icon, and prepare everything for the installer.
- Open `olympix-installer.iss` in Inno Setup and click Compile.
- The output will be `olympix_pattern_tool-installer.exe`.

---

## 🎨 Features

### 🏗️ Contract Management

- Load, save, and manage Solidity contracts
- View contract content with syntax highlighting
- CRUD operations for contract files
- User-friendly file organization

### 🔍 Pattern Management

- **Rust Patterns:** Save and manage Rust-based exploit patterns
- **Regex Patterns:** Save and manage regex patterns
- Organized pattern storage with type-specific directories
- Pattern type indicators and icons

### ⚡ Real-Time Regex Runner

- Write and test regex patterns on contracts without saving
- Full Rust regex syntax support
- Line-by-line match highlighting with context
- Built-in vulnerability pattern suggestions
- Instant results display

### 🎨 Modern GUI

- Dark theme with professional styling
- Tabbed interface for easy navigation
- Responsive design with split layouts
- Native Windows application feel
- Smooth animations and transitions

---

## 📋 Usage Guide

### Adding Your First Contract

1. Open the app
2. Go to **Contracts** tab
3. Click **"New Contract"**
4. Enter name (e.g., "MyToken")
5. Paste your Solidity code
6. Click **"Save Contract"**

### Testing with Regex

1. Go to **Regex Runner** tab
2. Select your contract from dropdown
3. Enter regex pattern (or use suggestions)
4. Click **"Run Regex"**
5. See results instantly!

### Creating Patterns

1. Go to **Patterns** tab
2. Click **"New Rust Pattern"** or **"New Regex Pattern"**
3. Enter pattern name and content
4. Save for later use

---

## 🛠️ Built-in Regex Suggestions

The tool includes pre-built patterns for common Solidity vulnerabilities:

- **Integer Overflow**: `(\w+)\s*\+\s*(\w+)`, `(\w+)\s*-\s*(\w+)`
- **Reentrancy**: `\.call\{.*\}\(`, `\.transfer\(`
- **Access Control**: `require\(msg\.sender\s*==\s*(\w+)\)`
- **Unchecked Calls**: `(\w+)\.call\(`, `(\w+)\.send\(`
- **Timestamp Dependence**: `block\.timestamp`, `now`
- **Delegate Calls**: `delegatecall\(`

---

## 🏗️ Project Structure

```
olympix_pattern_tool/
├── src/                      # Rust native GUI (eframe/egui)
│   ├── main.rs               # Application entry point
│   ├── hack_types_overflow.rs
│   └── ...                   # Other Rust modules
├── contracts/                # Sample Solidity contracts
├── patterns/                 # Sample exploit patterns
├── build-installer.ps1       # Automated build script
├── olympix-installer.iss     # Inno Setup script
├── favicon.ico                # Application icon
└── README.md                 # This file
```

---

## 🛠️ Troubleshooting

### App Won't Start

- ✅ Check Windows Defender isn't blocking it
- ✅ Run as Administrator if needed
- ✅ Ensure .NET Framework is installed

### Can't Find Files

- ✅ Check `Documents/Olympix Pattern Tool/` folder
- ✅ Create folders manually if missing
- ✅ Restart the application

### Performance Issues

- ✅ Close other applications
- ✅ Ensure 4GB+ RAM available
- ✅ Check disk space (500MB+ free)

---

## 📞 Support

- **Documentation**: See `INSTALLATION_GUIDE.md` for detailed instructions
- **Issues**: Create an issue in the repository
- **Features**: Request new features via issues

---

## 🔄 Updates

The app will check for updates automatically. To update:

1. Download the new installer
2. Run it (it will update the existing installation)
3. Restart the application

---

## 🎉 You're Ready!

The Olympix Pattern Tool is designed to be **completely user-friendly**. No coding experience required - just drag and drop your .sol files and start analyzing them with powerful regex patterns!

**For end users**: Download the installer and start using immediately!
**For developers**: Build the installer and distribute to your users!
