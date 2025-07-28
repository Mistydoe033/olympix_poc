use std::env;
use std::path::Path;

fn main() {
    // Only run on Windows
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        
        // Set the icon using absolute path
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let icon_path = Path::new(&manifest_dir).join("favicon.ico");
        
        if icon_path.exists() {
            res.set_icon(icon_path.to_str().unwrap());
            println!("cargo:rerun-if-changed={}", icon_path.display());
            println!("cargo:warning=Icon set successfully: {}", icon_path.display());
        } else {
            println!("cargo:warning=Icon file not found at: {}", icon_path.display());
            // Fallback to relative path
            res.set_icon("favicon.ico");
        }
        
        // Set version info to force icon replacement
        res.set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x01000000);
        res.set_version_info(winres::VersionInfo::FILEVERSION, 0x01000000);
        
        // Set comprehensive file properties to override defaults
        res.set("FileDescription", "Solidity Security Scanner");
        res.set("ProductName", "Olympix Pattern Tool");
        res.set("CompanyName", "Olympix Pattern Tool");
        res.set("LegalCopyright", "MIT License");
        res.set("OriginalFilename", "olympix-pattern-tool.exe");
        res.set("InternalName", "olympix-pattern-tool");
        res.set("FileVersion", "1.0.0.0");
        res.set("ProductVersion", "1.0.0.0");
        
        // Force icon replacement by setting explicit resource ID
        res.set_manifest_file("app.manifest");
        
        // Compile with error handling
        match res.compile() {
            Ok(_) => println!("cargo:warning=Windows resources compiled successfully"),
            Err(e) => {
                eprintln!("cargo:warning=Error compiling Windows resources: {}", e);
                // Try minimal compilation without version info
                let mut simple_res = winres::WindowsResource::new();
                if icon_path.exists() {
                    simple_res.set_icon(icon_path.to_str().unwrap());
                } else {
                    simple_res.set_icon("favicon.ico");
                }
                if let Err(e2) = simple_res.compile() {
                    eprintln!("cargo:warning=Even minimal resource compilation failed: {}", e2);
                }
            }
        }
    }
} 