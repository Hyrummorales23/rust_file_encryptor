// File encryption and decryption tool
// Uses XOR cipher for educational purposes
// Demonstrates Rust fundamentals: ownership, structs, vectors, file I/O

use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Struct to hold encryption settings
// Demonstrates object-oriented techniques with struct and impl
struct FileEncryptor {
    key: Vec<u8>,       // Encryption key as bytes
    mode: String,       // "encrypt" or "decrypt"
    buffer_size: usize, // Size of buffer for file processing
}

impl FileEncryptor {
    // Constructor - creates new FileEncryptor instance
    fn new(key: &str, mode: &str) -> Self {
        // Immutable variable - cannot change after assignment
        let default_buffer = 4096;

        // Mutable variable - we can modify this
        let mut buffer_size = default_buffer;

        // Expression - evaluates to a value
        let mode_string = mode.to_string();

        // Conditional - check if mode is valid
        if mode != "encrypt" && mode != "decrypt" {
            panic!("Mode must be 'encrypt' or 'decrypt'");
        }

        // Conditional expression - assign based on condition
        buffer_size = if mode == "encrypt" {
            4096 // Larger buffer for encryption
        } else {
            2048 // Smaller buffer for decryption
        };

        FileEncryptor {
            key: key.as_bytes().to_vec(),
            mode: mode_string,
            buffer_size,
        }
    }

    // Process file - reads, encrypts/decrypts, writes
    // Demonstrates function with references (&Path, &mut Vec)
    fn process_file(&self, input_path: &Path, output_path: &Path) -> io::Result<()> {
        // Read entire file into memory as Vec<u8>
        // Demonstrates data structure usage (Vec)
        let mut data = fs::read(input_path)?;

        // Process the data (encrypt or decrypt)
        // XOR is reversible: same operation for both
        self.xor_cipher(&mut data);

        // Write processed data to output file
        fs::write(output_path, &data)?;

        Ok(())
    }

    // XOR cipher implementation
    // Takes mutable reference to data Vec
    fn xor_cipher(&self, data: &mut Vec<u8>) {
        // Loop - iterate through each byte
        for (i, byte) in data.iter_mut().enumerate() {
            // XOR with key byte (cyclically)
            let key_byte = self.key[i % self.key.len()];
            *byte ^= key_byte;
        }
    }

    // Process directory - encrypt/decrypt all .txt files
    fn process_directory(&self, dir_path: &Path) -> io::Result<()> {
        // Read directory entries
        let entries = fs::read_dir(dir_path)?;

        // Loop - iterate over files in directory
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Conditional - check if it's a file and has .txt extension
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "txt" {
                        // Create output filename
                        let mut output_name = path.file_stem().unwrap().to_os_string();
                        output_name.push(if self.mode == "encrypt" {
                            "_encrypted.txt"
                        } else {
                            "_decrypted.txt"
                        });

                        let output_path = path.parent().unwrap().join(output_name);

                        println!(
                            "Processing: {:?} -> {:?}",
                            path.file_name().unwrap(),
                            output_path.file_name().unwrap()
                        );
                        self.process_file(&path, &output_path)?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    println!("========================================");
    println!("   Rust File Encryption/Decryption Tool");
    println!("========================================\n");

    // Get user input
    println!("Enter mode (encrypt/decrypt):");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode)?;
    let mode = mode.trim();

    println!("Enter encryption key (any text):");
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let key = key.trim();

    println!("Enter file or directory path:");
    let mut path_str = String::new();
    io::stdin().read_line(&mut path_str)?;
    let path_str = path_str.trim();

    let path = Path::new(path_str);

    // Create encryptor instance
    let encryptor = FileEncryptor::new(key, mode);

    // Conditional - check if path is file or directory
    if path.is_file() {
        // Process single file
        let mut output_name = path.file_stem().unwrap().to_os_string();
        output_name.push(if mode == "encrypt" {
            "_encrypted.txt"
        } else {
            "_decrypted.txt"
        });

        let output_path = path.parent().unwrap().join(output_name);

        println!("\nProcessing single file...");
        encryptor.process_file(path, &output_path)?;
        println!("✓ Done! Output: {:?}", output_path.file_name().unwrap());
    } else if path.is_dir() {
        // Process entire directory
        println!("\nProcessing all .txt files in directory...");
        encryptor.process_directory(path)?;
        println!("✓ Done! All files processed.");
    } else {
        println!("✗ Error: Path does not exist!");
    }

    println!("\nThank you for using Rust File Encryptor!");
    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_encryptor_creation() {
        let encryptor = FileEncryptor::new("test123", "encrypt");
        assert_eq!(encryptor.mode, "encrypt");
        assert_eq!(encryptor.key, b"test123");
    }

    #[test]
    fn test_xor_cipher() {
        let encryptor = FileEncryptor::new("key", "encrypt");
        let mut data = vec![1, 2, 3, 4];
        let original = data.clone();

        // Encrypt
        encryptor.xor_cipher(&mut data);
        assert_ne!(data, original);

        // Decrypt (XOR twice with same key)
        encryptor.xor_cipher(&mut data);
        assert_eq!(data, original);
    }
}
