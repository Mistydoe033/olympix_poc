# Olympix Pattern Tool - Installation Guide

## 🚀 Quick Start (For End Users)

### What You Get

Olympix Pattern Tool is a **standalone Windows application** that includes:

- ✅ **No coding required** - Just install and run
- ✅ **Modern native interface** - User-friendly dark theme
- ✅ **Rust backend** - Fast and secure pattern analysis
- ✅ **Sample contracts and patterns** - Ready to use examples
- ✅ **Real-time regex testing** - Test patterns on your .sol files instantly

### 📦 Installation Steps

#### Option 1: Download Pre-built Installer (Recommended)

1. **Download** the `Olympix Pattern Tool Setup.msi` file
2. **Double-click** the installer file
3. **Follow** the installation wizard
4. **Launch** from Desktop or Start Menu
5. **Start using** immediately!

#### Option 2: Build from Source (For Developers)

If you want to build the installer yourself:

1. **Prerequisites**:

   - Windows 10/11
   - Rust (latest stable)
   - Git

2. **Build Steps**:

   ```powershell
   # Clone the repository
   git clone <repository-url>
   cd olympix_pattern_tool

   # Run the build script
   .\build-installer.ps1
   ```

   Or, to build manually:

   ```powershell
   cargo build --release
   ```

3. **Find the installer or binary** at:
   ```
   target\release\olympix-pattern-tool.exe
   ```

### 🎯 What the Installer Does

The installer automatically:

- 📁 **Installs** the application to `Program Files`
- 🖥️ **Creates** desktop shortcut
- 📋 **Adds** to Start Menu
- 📂 **Sets up** folders:
  - `Documents/Olympix Pattern Tool/contracts/` - For your .sol files
  - `Documents/Olympix Pattern Tool/patterns/rust/` - For Rust patterns
  - `Documents/Olympix Pattern Tool/patterns/regex/` - For regex patterns
- 🔧 **Configures** everything for immediate use

### 📁 File Structure After Installation

```
C:\Program Files\Olympix Pattern Tool\
├── olympix-pattern-tool.exe    # Main application
├── contracts\                   # Sample contracts
├── patterns\                    # Sample patterns
│   ├── rust\
│   └── regex\
└── resources\                   # Application resources

Documents\Olympix Pattern Tool\
├── contracts\                   # Your contracts go here
├── patterns\
│   ├── rust\                    # Your Rust patterns
│   └── regex\                   # Your regex patterns
└── config.json                  # App configuration
```

### 🚀 First Launch

1. **Double-click** the desktop shortcut
2. **The app opens** with a modern dark interface
3. **Three tabs** are available:
   - **Contracts**: Manage your .sol files
   - **Patterns**: Create and save exploit patterns
   - **Regex Runner**: Test patterns on contracts in real-time

### 📋 Using the App

#### Adding Your First Contract

1. Click **"New Contract"** in the Contracts tab
2. Enter a name (e.g., "MyToken")
3. Paste your Solidity code
4. Click **"Save Contract"**

#### Testing with Regex

1. Go to **Regex Runner** tab
2. Select your contract from the dropdown
3. Enter a regex pattern (or use suggestions)
4. Click **"Run Regex"**
5. See results instantly!

#### Creating Patterns

1. Go to **Patterns** tab
2. Click **"New Rust Pattern"** or **"New Regex Pattern"**
3. Enter pattern name and content
4. Save for later use

### 🛠️ Troubleshooting

#### App Won't Start

- ✅ Check Windows Defender isn't blocking it
- ✅ Run as Administrator if needed
- ✅ Ensure .NET Framework is installed

#### Can't Find Files

- ✅ Check `Documents\Olympix Pattern Tool\` folder
- ✅ Create folders manually if missing
- ✅ Restart the application

#### Performance Issues

- ✅ Close other applications
- ✅ Ensure 4GB+ RAM available
- ✅ Check disk space (500MB+ free)

### 📞 Support

- **Documentation**: See README.md for detailed usage
- **Issues**: Create an issue in the repository
- **Features**: Request new features via issues

### 🔄 Updates

The app will check for updates automatically. To update:

1. Download the new installer
2. Run it (it will update the existing installation)
3. Restart the application

---

## 🎉 You're Ready!

The Olympix Pattern Tool is now installed and ready to use. No coding experience required - just drag and drop your .sol files and start analyzing them with powerful regex patterns!
