use libc::{tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW};
use std::fs::{File, OpenOptions};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::prelude::*;
use std::io::{self, stdin, BufRead, BufReader, Read, Write};
use std::os::unix::io::AsRawFd;

fn main() {
    loop {
        let mut response = String::new();
        get_input(
            r#"Welcome to Cruz Password Manager
press 1 to create an account
press 2 to login to an account
press q to quit
"#,
            &mut response,
        );

        let mut key = String::new();
        let mut user_name = String::new();
        if response == "1" {
            signup(&mut key).expect("should be handled in the code");
        } else if response == "2" {
            let login_sesh = login(&mut key, &mut user_name);
            if login_sesh.is_ok() {
                user_session(&key, &user_name);
            } else {
                println!("{}", login_sesh.err().unwrap())
            };
        } else if response.to_lowercase() == "q" {
            println!("Thanks for using the Password Manager");
            break;
        } else {
            println!("{response} is not an option");
        }
    }
}

fn signup(key: &mut String) -> Result<String, ()> {
    let mut username = String::new();
    get_input("Input your username", &mut username);

    let mut password = String::new();
    let mut password_hasher = DefaultHasher::new();
    get_masked_input("Please input a password", &mut password);

    get_symmetric_key(key, &password);
    password.hash(&mut password_hasher);
    password = password_hasher.finish().to_string();
    println!("\nMaster account {username} created");

    let user = username + " " + password.as_str() + "\n";

    let database = "master.db";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(database)
        .unwrap();
    file.write_all(user.as_bytes()).unwrap();
    Ok(key.to_string())
}

fn login<'a>(
    key: &'a mut String,
    user_name: &mut String,
) -> Result<&'a mut String, Box<dyn std::error::Error>> {
    let mut username = String::new();
    let mut password = String::new();

    get_input("Input your username", &mut username);
    get_masked_input("Input your password", &mut password);

    get_symmetric_key(key, &password);
    let mut password_hasher = DefaultHasher::new();
    password.hash(&mut password_hasher);
    password = password_hasher.finish().to_string();

    let database = "master.db";
    let file = File::open(database)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let u: Vec<_> = line.split(" ").collect();
        if username == u[0] && password == u[1] {
            *user_name = username.clone();
            println!("\nWelcome back {username}");
            return Ok(key);
        }
    }

    Err("Can't find your account".into())
}

fn get_symmetric_key(key: &mut String, password: &String) {
    hash_key(key, password)
}

fn hash_key(key: &mut String, password: &String) {
    let mut hash: u128 = 5381;
    let password = password.clone().into_bytes().into_iter();
    for i in password {
        hash = hash.wrapping_mul(33).wrapping_add(i.into());
    }
    *key = hash.to_string();
}

fn user_session(key: &String, user: &String) {
    loop {
        let mut response = String::new();
        get_input(
            "press 1 to add a password\npress 2 to retrieve a password\npress b to go back",
            &mut response,
        );

        if response == "1" {
            add_password(key, user);
        } else if response == "2" {
            retrieve_password(key, user).unwrap();
        } else if response == "b" {
            break;
        } else {
            println!("{response} is not an option");
        }
    }
}

fn add_password(key: &String, user: &String) {
    let mut account = String::new();
    get_input(
        "\nWhat is the name of the account associated with the password",
        &mut account,
    );

    let mut password = String::new();
    println!("Type in the passowrd");
    get_masked_input("Type in the password", &mut password);

    let password = encrypt_password(key, &password);
    let response = account.clone() + " " + password.as_str() + "\n";

    let database = user.clone() + "_database.db";
    let database = database.as_str();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(database)
        .unwrap();
    file.write_all(response.as_bytes()).unwrap();

    println!("encryped password is {password}");
    println!("password added to {account} account");
}

fn retrieve_password(key: &String, user: &String) -> Result<(), io::Error> {
    let mut account = String::new();
    get_input(
        "\nWhat is the name of the account associated with the password",
        &mut account,
    );

    let database = user.clone() + "_database.db";
    let database = database.as_str();

    let file = File::open(database)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{line}");
        let u: Vec<_> = line.split(" ").collect();
        if account == u[0] {
            let mut password = u[1].to_string();
            password = decrypt_password(key, &password);
            println!("The password for {account} is {password}");
        }
    }
    Ok(())
}

fn encrypt_password(key: &String, password: &String) -> String {
    let mut p: Vec<u32> = Vec::new();
    let mut count: u32 = 0;
    let key_char = key.as_bytes().to_vec();
    for i in password.as_bytes() {
        p.push(*i as u32 * key_char[count as usize] as u32);
        count += 1;
        if count >= key_char.len() as u32 {
            count = 0;
        }
    }
    let mut enc_password = String::new();
    for (i, items) in p.iter().enumerate() {
        if i > 0 {
            enc_password.push('-');
        }
        enc_password.push_str(items.to_string().as_str());
    }
    enc_password
}
fn decrypt_password(key: &String, password: &String) -> String {
    let mut p = String::new();
    let mut count: u32 = 0;
    let key_char = key.as_bytes();
    for num_str in password.split('-') {
        println!("{num_str}");
        let num: u32 = num_str.parse().expect("Should be a number");
        let orig_char: u32 = num / key_char[count as usize] as u32;
        p.push(orig_char as u8 as char);
        count += 1;
        if count >= key_char.len() as u32 {
            count = 0;
        }
    }
    p
}

fn set_raw_mode(enable: bool) {
    unsafe {
        let fd = io::stdin().as_raw_fd();
        let mut term: termios = std::mem::zeroed();
        tcgetattr(fd, &mut term);

        if enable {
            term.c_lflag &= !(ICANON | ECHO);
        } else {
            term.c_lflag |= ICANON | ECHO;
        }

        tcsetattr(fd, TCSANOW, &term);
    }
}

fn get_masked_input(query: &str, input: &mut String) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    *input = String::new();

    println!("{query}");
    stdout.flush().unwrap();

    set_raw_mode(true);

    let mut buf = [0u8; 1];
    loop {
        stdin.read_exact(&mut buf).unwrap();
        let ch = buf[0] as char;

        if ch == '\n' || ch == '\r' {
            break;
        } else if ch == '\x08' || ch == '\x7f' {
            if !input.is_empty() {
                input.pop();
                print!("\x08 \x20 \x08");
                stdout.flush().unwrap();
            }
        } else {
            input.push(ch);
            print!("*");
            stdout.flush().unwrap();
        }
    }

    set_raw_mode(false);
    println!("\nthe input typed is {input}");
}

fn get_input(query: &str, output: &mut String) {
    println!("{query}");
    stdin().read_line(output).unwrap();
    *output = output.trim().to_string();
}
