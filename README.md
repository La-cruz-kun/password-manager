# Cruz Password Manager

A simple command-line password manager written in Rust.  
It allows you to:

- Create a master account with a username and password  
- Log into your account securely  
- Store and retrieve credentials (account name, username, and password)  
- Manage stored entries in a local database file  

## Features

- **Account Creation**: Create a master account stored in `master.db`  
- **Login System**: Authenticate with username and password  
- **Password Security**: Passwords are hashed before being stored  
- **Symmetric Key**: Derived from your master password for encrypting stored data  
- **Local Database**: All data is stored in plain files (`master.db`, user-specific `.db` files)  

## How It Works

1. On startup, you are given three options:
   - `1` → Create a new master account  
   - `2` → Log in with an existing account  
   - `q` → Quit  

2. After logging in, you enter a **user session**, where you can:
   - Store credentials (website/app + username + password)  
   - Retrieve stored credentials  
   - Exit back to the main menu  

## Installation

### Prerequisites
- Rust (latest stable) → [Install Rust](https://www.rust-lang.org/tools/install)  
- Linux/macOS (uses `libc` for terminal input handling)  

### Clone and Build
```bash
git clone https://github.com/yourusername/cruz-password-manager.git
cd cruz-password-manager
cargo build --release

### Run
```bash
cargo run

## Usage Example

```text
Welcome to Cruz Password Manager
press 1 to create an account
press 2 to login to an account
press q to quit
If you choose `1`, you’ll be prompted to create a username and password.  
If you choose `2`, you’ll log in and can start storing credentials.  

## File Structure

- `master.db` → Stores master accounts (username + hashed password)  
- `<username>.db` → Stores encrypted credentials for each user  

## Security Notes

- Passwords are hashed using Rust’s `DefaultHasher`  

  > **Warning**: `DefaultHasher` is **not cryptographically secure**.  
  > For better protection, replace it with **Argon2**, **bcrypt**, or **PBKDF2**.  

- Encryption is based on a symmetric key derived from your master password.  

- This project is intended for **educational purposes** and is **not recommended for storing sensitive data** without further improvements.  
