use std::fs;
use std::path::{Path, PathBuf};
use crate::models::{ContractInfo, PatternInfo, FileType};

pub fn get_app_data_dir() -> PathBuf {
    let documents_dir = dirs::document_dir().unwrap_or_else(|| PathBuf::from("."));
    documents_dir.join("Olympix Pattern Tool")
}

pub fn get_contracts_dir() -> PathBuf {
    get_app_data_dir().join("contracts")
}

pub fn get_patterns_dir() -> PathBuf {
    get_app_data_dir().join("patterns")
}

pub fn get_rust_patterns_dir() -> PathBuf {
    get_patterns_dir().join("rust")
}

pub fn get_regex_patterns_dir() -> PathBuf {
    get_patterns_dir().join("regex")
}

pub fn ensure_directories_exist() -> Result<(), std::io::Error> {
    let dirs = [
        get_app_data_dir(),
        get_contracts_dir(),
        get_patterns_dir(),
        get_rust_patterns_dir(),
        get_regex_patterns_dir(),
    ];
    
    for dir in dirs {
        fs::create_dir_all(&dir)?;
    }
    
    // Copy sample files if they don't exist
    copy_sample_files()?;
    
    Ok(())
}

pub fn list_contracts() -> Vec<ContractInfo> {
    let mut contracts = Vec::new();
    let contracts_dir = get_contracts_dir();
    
    if let Ok(entries) = fs::read_dir(contracts_dir) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "sol" {
                    if let Some(name) = entry.file_name().to_str() {
                        if let Ok(metadata) = entry.metadata() {
                            contracts.push(ContractInfo {
                                name: name.to_string(),
                                path: entry.path(),
                                size: metadata.len(),
                                last_modified: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now()),
                            });
                        }
                    }
                }
            }
        }
    }
    
    contracts
}

pub fn list_patterns() -> Vec<PatternInfo> {
    let mut patterns = Vec::new();
    
    // List Rust patterns
    let rust_dir = get_rust_patterns_dir();
    if let Ok(entries) = fs::read_dir(rust_dir) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    if let Some(name) = entry.file_name().to_str() {
                        if let Ok(metadata) = entry.metadata() {
                            patterns.push(PatternInfo {
                                name: name.to_string(),
                                path: entry.path(),
                                pattern_type: FileType::RustPattern,
                                description: format!("Rust pattern for {}", name.replace(".rs", "")),
                                size: metadata.len(),
                                last_modified: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now()),
                            });
                        }
                    }
                }
            }
        }
    }
    
    // List Regex patterns
    let regex_dir = get_regex_patterns_dir();
    if let Ok(entries) = fs::read_dir(regex_dir) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "regex" || ext == "txt" {
                    if let Some(name) = entry.file_name().to_str() {
                        if let Ok(metadata) = entry.metadata() {
                            patterns.push(PatternInfo {
                                name: name.to_string(),
                                path: entry.path(),
                                pattern_type: FileType::RegexPattern,
                                description: format!("Regex pattern for {}", name.replace(".regex", "").replace(".txt", "")),
                                size: metadata.len(),
                                last_modified: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now()),
                            });
                        }
                    }
                }
            }
        }
    }
    
    patterns
}

pub fn import_file(source_path: &Path, file_type: &FileType) -> Result<PathBuf, std::io::Error> {
    let destination_dir = match file_type {
        FileType::Contract => get_contracts_dir(),
        FileType::RustPattern => get_rust_patterns_dir(),
        FileType::RegexPattern => get_regex_patterns_dir(),
    };
    
    let file_name = source_path.file_name().unwrap_or_default();
    let destination = destination_dir.join(file_name);
    
    // Handle duplicate files
    let mut final_destination = destination.clone();
    let mut counter = 1;
    while final_destination.exists() {
        let stem = source_path.file_stem().unwrap_or_default();
        let ext = source_path.extension().unwrap_or_default();
        let new_name = format!("{}_{}.{}", 
            stem.to_str().unwrap_or("file"), 
            counter, 
            ext.to_str().unwrap_or(""));
        final_destination = destination_dir.join(new_name);
        counter += 1;
    }
    
    fs::copy(source_path, &final_destination)?;
    Ok(final_destination)
}

pub fn import_folder(source_path: &Path, file_type: &FileType) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut imported_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(source_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_file() {
                let should_import = match file_type {
                    FileType::Contract => path.extension().map_or(false, |ext| ext == "sol"),
                    FileType::RustPattern => path.extension().map_or(false, |ext| ext == "rs"),
                    FileType::RegexPattern => path.extension().map_or(false, |ext| ext == "regex" || ext == "txt"),
                };
                
                if should_import {
                    match import_file(&path, file_type) {
                        Ok(dest) => imported_files.push(dest),
                        Err(e) => eprintln!("Failed to import {}: {}", path.display(), e),
                    }
                }
            }
        }
    }
    
    Ok(imported_files)
}

pub fn read_file_content(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn write_file_content(path: &Path, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}

pub fn export_report(content: &str, filename: &str) -> Result<(), std::io::Error> {
    let app_dir = get_app_data_dir();
    let report_path = app_dir.join(filename);
    if let Some(parent) = report_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    fs::write(report_path, content)
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    match bytes {
        0..KB => format!("{} B", bytes),
        KB..MB => format!("{:.1} KB", bytes as f64 / KB as f64),
        MB..GB => format!("{:.1} MB", bytes as f64 / MB as f64),
        _ => format!("{:.1} GB", bytes as f64 / GB as f64),
    }
}

pub fn format_timestamp(time: std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;
    
    if let Ok(duration) = time.duration_since(UNIX_EPOCH) {
        let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0);
        if let Some(dt) = datetime {
            dt.format("%Y-%m-%d %H:%M").to_string()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}

fn copy_sample_files() -> Result<(), std::io::Error> {
    // Get the current executable directory to find sample files
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));
    
    // Try to find sample files relative to the executable
    let sample_contracts_dir = exe_dir.join("contracts");
    let sample_patterns_dir = exe_dir.join("patterns");
    
    // Copy sample contracts if they exist
    if sample_contracts_dir.exists() {
        copy_directory_contents(&sample_contracts_dir, &get_contracts_dir())?;
    }
    
    // Copy sample patterns if they exist
    if sample_patterns_dir.exists() {
        copy_directory_contents(&sample_patterns_dir, &get_patterns_dir())?;
    }
    
    // For development, also try to copy from project directory
    #[cfg(debug_assertions)]
    {
        let project_contracts_dir = Path::new("contracts");
        let project_patterns_dir = Path::new("patterns");
        
        if project_contracts_dir.exists() {
            copy_directory_contents(project_contracts_dir, &get_contracts_dir())?;
        }
        
        if project_patterns_dir.exists() {
            copy_directory_contents(project_patterns_dir, &get_patterns_dir())?;
        }
    }
    
    // Always try to copy from current working directory as fallback
    let cwd = std::env::current_dir()?;
    let cwd_contracts_dir = cwd.join("contracts");
    let cwd_patterns_dir = cwd.join("patterns");
    
    if cwd_contracts_dir.exists() {
        copy_directory_contents(&cwd_contracts_dir, &get_contracts_dir())?;
    }
    
    if cwd_patterns_dir.exists() {
        copy_directory_contents(&cwd_patterns_dir, &get_patterns_dir())?;
    }
    
    Ok(())
}

fn copy_directory_contents(source_dir: &Path, dest_dir: &Path) -> Result<(), std::io::Error> {
    if let Ok(entries) = fs::read_dir(source_dir) {
        for entry in entries.flatten() {
            let source_path = entry.path();
            let file_name = source_path.file_name().unwrap_or_default();
            let dest_path = dest_dir.join(file_name);
            
            // Only copy if destination doesn't exist
            if !dest_path.exists() {
                if source_path.is_file() {
                    fs::copy(&source_path, &dest_path)?;
                } else if source_path.is_dir() {
                    fs::create_dir_all(&dest_path)?;
                    copy_directory_contents(&source_path, &dest_path)?;
                }
            }
        }
    }
    Ok(())
} 