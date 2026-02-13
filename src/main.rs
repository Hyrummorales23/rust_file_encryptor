// File encryption and decryption tool
// Encrypts and decrypts files in place (overwrites original file)
// Uses XOR cipher for educational purposes

use std::fs;
use std::io::{self};
use std::path::Path;

// Struct to hold encryption settings
struct FileEncryptor {
    key: Vec<u8>, // Encryption key as bytes
}

impl FileEncryptor {
    // Constructor - creates new FileEncryptor instance
    fn new(key: &str, _mode: &str) -> Self {
        // Added underscore to tell Rust "I know this param isn't used"
        // Conditional - check if mode is valid
        if _mode != "encrypt" && _mode != "decrypt" {
            panic!("Mode must be 'encrypt' or 'decrypt'");
        }

        FileEncryptor {
            key: key.as_bytes().to_vec(),
        } // No comma here, just closing brace
    } // Closing brace for new()

    // Encrypt or decrypt a file IN PLACE (overwrites the original)
    fn process_file_in_place(&self, file_path: &Path) -> io::Result<()> {
        // Read file into memory
        let mut data = fs::read(file_path)?;

        // Process the data (XOR encryption/decryption)
        self.xor_cipher(&mut data);

        // Write back to SAME file (overwrite)
        fs::write(file_path, &data)?;

        println!(
            "  ✓ Processed: {}",
            file_path.file_name().unwrap().to_string_lossy()
        );
        Ok(())
    }

    // XOR cipher implementation
    fn xor_cipher(&self, data: &mut Vec<u8>) {
        for (i, byte) in data.iter_mut().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            *byte ^= key_byte;
        }
    }

    // Process directory - encrypt/decrypt all .txt files in place
    fn process_directory(&self, dir_path: &Path) -> io::Result<()> {
        let entries = fs::read_dir(dir_path)?;

        println!("\nProcessing all .txt files in directory...");

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Only process .txt files
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "txt" {
                        self.process_file_in_place(&path)?;
                    }
                }
            }
        }

        println!("\n✓ All files processed successfully!");
        Ok(())
    }
}

fn main() -> io::Result<()> {
    println!("========================================");
    println!("   Rust File Encryption/Decryption Tool");
    println!("         (In-Place Mode)");
    println!("========================================\n");

    println!("⚠️  WARNING: This will OVERWRITE your original files!");
    println!("   Make backups if needed.\n");

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

    // Confirmation
    println!("\n⚠️  This will modify files in place. Continue? (yes/no):");
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    let confirm = confirm.trim();

    if confirm != "yes" && confirm != "y" {
        println!("Operation cancelled.");
        return Ok(());
    }

    // Create encryptor instance
    let encryptor = FileEncryptor::new(key, mode);

    // Process file or directory
    if path.is_file() {
        println!("\nProcessing single file...");
        encryptor.process_file_in_place(path)?;
        println!("\n✓ File processed successfully!");
    } else if path.is_dir() {
        encryptor.process_directory(path)?;
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

    #[test]
    fn test_encryptor_creation() {
        let encryptor = FileEncryptor::new("test123", "encrypt");
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

        // Decrypt (XOR twice)
        encryptor.xor_cipher(&mut data);
        assert_eq!(data, original);
    }
}
