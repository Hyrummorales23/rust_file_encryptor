# Rust File Encryption Tool

A professional-grade file encryption and decryption tool built with Rust. This application demonstrates secure file handling, XOR cipher implementation, and Rust's powerful memory safety features.

## Features

- 🔐 **File Encryption/Decryption**: Secure your sensitive text files
- 📁 **Batch Processing**: Encrypt/decrypt all .txt files in a directory
- 🔑 **Custom Keys**: Use any text as your encryption key
- 🛡️ **Memory Safe**: Leverages Rust's ownership model for security
- 📊 **Real-time Feedback**: See progress as files are processed

## Demo Video

[Watch the demo](https://youtu.be/iWvN0LG_nco)

## Technologies Used

- **Language**: Rust 2021 Edition
- **Build Tool**: Cargo
- **Dependencies**: None (pure Rust standard library)
- **IDE**: VS Code with rust-analyzer

## How It Works

This tool uses XOR cipher for encryption/decryption:

1. Reads file as bytes into a Vec<u8>
2. Applies XOR operation with your key
3. Saves the processed data to a new file

**Why XOR?** It's reversible - same operation encrypts and decrypts!

## Installation

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/rust-file-encryptor.git
cd rust-file-encryptor

# Build release version
cargo build --release

# Run the tool
./target/release/rust_file_encryptor
```
