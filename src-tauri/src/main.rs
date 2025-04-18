#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use rayon::prelude::*;
use tokio::task;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[tauri::command]
async fn scan_folder(folder_path: String, hash_file: String) -> Result<Vec<String>, String> {
    static BAD_HASHES_CACHE: Lazy<Mutex<Option<HashSet<String>>>> = Lazy::new(|| Mutex::new(None));

    fn sha256_file(filename: &str) -> std::io::Result<String> {
        let mut file = File::open(filename)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn is_exe_or_dll_file(path: &std::path::Path) -> bool {
        matches!(path.extension().and_then(|e| e.to_str()), Some("exe") | Some("dll"))
    }

    fn load_hashes(path: &str) -> Result<HashSet<String>, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut hashes = HashSet::new();

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                hashes.insert(trimmed.to_string());
            }
        }
        Ok(hashes)
    }

    // Extract cache safely before await
    let bad_hashes = {
        let maybe_cached = {
            let cache = BAD_HASHES_CACHE.lock().unwrap();
            cache.clone()
        };

        if let Some(hashes) = maybe_cached {
            hashes
        } else {
            let hashes = load_hashes(&hash_file)?;
            let mut cache = BAD_HASHES_CACHE.lock().unwrap();
            *cache = Some(hashes.clone());
            hashes
        }
    };

    let results: Vec<String> = task::spawn_blocking(move || {
        WalkDir::new(&folder_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file() && is_exe_or_dll_file(e.path()))
            .par_bridge()
            .map(|entry| {
                let path = entry.path();
                match sha256_file(path.to_str().unwrap()) {
                    Ok(hash) => {
                        if bad_hashes.contains(&hash) {
                            format!("🚨 MALWARE DETECTED: {} (hash: {})", path.display(), hash)
                        } else {
                            format!("✅ Clean: {}", path.display())
                        }
                    }
                    Err(e) => format!("❌ Error reading {}: {}", path.display(), e),
                }
            })
            .collect()
    }).await.map_err(|e| format!("Threading error: {}", e))?;

    Ok(results)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![scan_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
