use lopdf::Document;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::fs::{OpenOptions, rename};
use std::io::Write;
use std::path::Path;

pub fn run_password_cracker(filename: &str, pattern: &str, num_threads: usize) -> Result<String, String> {
    // Handle log file - rename if exists
    let log_filename = "password_attempts.log";
    if Path::new(log_filename).exists() {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("password_attempts_{}.log", timestamp);
        let _ = rename(log_filename, &backup_name);
    }
    
    // Create log file
    let log_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_filename)
            .map_err(|e| format!("Failed to create log file: {}", e))?
    ));
    
    writeln!(log_file.lock().unwrap(), "=== PDF Password Cracker Log ===").ok();
    writeln!(log_file.lock().unwrap(), "File: {}", filename).ok();
    writeln!(log_file.lock().unwrap(), "Pattern: {}", pattern).ok();
    writeln!(log_file.lock().unwrap(), "Threads: {}", num_threads).ok();
    writeln!(log_file.lock().unwrap(), "Started: {:?}\n", chrono::Local::now()).ok();
    
    // Configure thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .ok();
    
    // Load PDF
    let doc_template = Document::load(filename)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    if !doc_template.is_encrypted() {
        return Err("PDF is not encrypted!".to_string());
    }
    
    // Parse pattern
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let mut unknown_positions = Vec::new();
    let mut char_sets = Vec::new();
    
    for (i, &ch) in pattern_chars.iter().enumerate() {
        match ch {
            'n' => {
                unknown_positions.push(i);
                char_sets.push(vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
            },
            'c' => {
                unknown_positions.push(i);
                let mut chars = Vec::new();
                for c in 'a'..='z' { chars.push(c); }
                for c in 'A'..='Z' { chars.push(c); }
                char_sets.push(chars);
            },
            'a' => {
                unknown_positions.push(i);
                let mut chars = Vec::new();
                for c in '0'..='9' { chars.push(c); }
                for c in 'a'..='z' { chars.push(c); }
                for c in 'A'..='Z' { chars.push(c); }
                char_sets.push(chars);
            },
            'x' => {
                unknown_positions.push(i);
                let mut chars = Vec::new();
                for c in 0x20..=0x7E { chars.push(c as u8 as char); }
                char_sets.push(chars);
            },
            _ => {}
        }
    }
    
    if unknown_positions.is_empty() {
        return Err("Pattern has no unknown characters!".to_string());
    }
    
    let total_combinations: u64 = char_sets.iter()
        .map(|set| set.len() as u64)
        .product();
    
    writeln!(log_file.lock().unwrap(), "Total combinations: {}\n", total_combinations).ok();
    
    let chunk_size = (total_combinations / 1000).max(100).min(100_000) as usize;
    let num_chunks = ((total_combinations as f64) / (chunk_size as f64)).ceil() as u64;
    
    let start_time = Instant::now();
    let found = Arc::new(AtomicBool::new(false));
    let attempts_counter = Arc::new(AtomicU64::new(0));
    
    let pattern_chars_arc = Arc::new(pattern_chars.clone());
    let unknown_positions_arc = Arc::new(unknown_positions.clone());
    let char_sets_arc = Arc::new(char_sets.clone());
    
    let found_password = Arc::new(Mutex::new(String::new()));
    let found_password_clone = found_password.clone();
    
    // Process chunks in parallel
    (0..num_chunks).into_par_iter().for_each(|chunk_idx| {
        if found.load(Ordering::Relaxed) {
            return;
        }
        
        let start = chunk_idx * (chunk_size as u64);
        let end = (start + chunk_size as u64).min(total_combinations);
        
        for combo_idx in start..end {
            if found.load(Ordering::Relaxed) {
                return;
            }
            
            let mut password_chars = pattern_chars_arc.as_ref().clone();
            let mut remaining = combo_idx;
            
            for (pos_idx, &position) in unknown_positions_arc.iter().enumerate().rev() {
                let char_set = &char_sets_arc[pos_idx];
                let char_idx = (remaining % (char_set.len() as u64)) as usize;
                password_chars[position] = char_set[char_idx];
                remaining /= char_set.len() as u64;
            }
            
            let password: String = password_chars.iter().collect();
            
            // Log progress
            let attempt_num = attempts_counter.fetch_add(1, Ordering::Relaxed);
            if attempt_num % 10000 == 0 {
                if let Ok(mut file) = log_file.lock() {
                    let _ = writeln!(file, "Progress: {} attempts, Current: {}", attempt_num, password);
                }
            }
            
            let mut doc = doc_template.clone();
            
            if doc.decrypt(password.as_bytes()).is_ok() {
                found.store(true, Ordering::SeqCst);
                *found_password_clone.lock().unwrap() = password.clone();
                
                if let Ok(mut file) = log_file.lock() {
                    let _ = writeln!(file, "\n{}", "=".repeat(60));
                    let _ = writeln!(file, "*** SUCCESS! PASSWORD FOUND! ***");
                    let _ = writeln!(file, "{}", "=".repeat(60));
                    let _ = writeln!(file, "*** PASSWORD: {} ***", password);
                    let _ = writeln!(file, "{}", "=".repeat(60));
                    let _ = writeln!(file, "Found at: {:?}", chrono::Local::now());
                    let _ = writeln!(file, "Time elapsed: {:.2?}", start_time.elapsed());
                    let _ = writeln!(file, "Attempts made: {}", attempt_num);
                }
                
                let output_path = filename.replace(".pdf", "_unlocked.pdf");
                let _ = doc.save(output_path);
                return;
            }
        }
    });
    
    if found.load(Ordering::Relaxed) {
        Ok(found_password.lock().unwrap().clone())
    } else {
        Err("Password not found in all combinations".to_string())
    }
}
