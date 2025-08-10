use std::fs::{File, OpenOptions};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::prelude::*;
use std::io::{self, stdin, BufRead, BufReader};

fn main() {
    loop {
        println!("Welcome to Cruz Password Manager");
        println!("press 1 to create an account");
        println!("press 2 to login to an account");
        println!("press q to quit");
        let mut response = String::new();
        stdin().read_line(&mut response).unwrap();
        response = response.trim().to_string();

        let mut key = String::new();
        let mut user_name = String::new();
        if response == "1" {
            signup(&mut key).expect("should be handled in the code");
        } else if response == "2" {
            if login(&mut key, &mut user_name).is_ok() {
                user_session(&key, &user_name);
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
    println!("Please input a username");
    stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();

    println!("Please input a password");
    let mut password = String::new();
    let mut password_hasher = DefaultHasher::new();
    stdin().read_line(&mut password).unwrap();
    password = password.trim().to_string();

    get_symmetric_key(key, &password);
    password.hash(&mut password_hasher);
    password = password_hasher.finish().to_string();
    println!("Master account {username} created");

    let user = username + " " + password.as_str();

    let database = "master.db";
    std::fs::write(database, &user).unwrap_or({
        std::fs::File::create(database).unwrap();
        std::fs::write(database, &user).unwrap();
    });
    Ok(key.to_string())
}

fn login<'a>(key: &'a mut String, user_name: &mut String) -> Result<&'a mut String, io::Error> {
    let mut username = String::new();
    let mut password = String::new();

    println!("input your username");
    stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();
    println!("input your password");

    stdin().read_line(&mut password).unwrap();
    password = password.trim().to_string();

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
            println!("Welcome back {username}");
            println!("You key is {key}");
        }
    }
    *user_name = username.clone();

    Ok(key)
}

fn get_symmetric_key(key: &mut String, password: &String) {
    hash_key(key, password)
}

fn hash_key(key: &mut String, password: &String) {
    let mut hash: u32 = 5381;
    let password = password.clone().into_bytes().into_iter();
    for i in password {
        hash = ((hash << 5) + hash) + i as u32;
    }
    *key = hash.to_string();
}

fn user_session(key: &String, user: &String) {
    loop {
        println!("press 1 to add a password");
        println!("press 2 to retrieve a password");
        println!("press b to go back");
        let mut response = String::new();
        stdin().read_line(&mut response).unwrap();
        response = response.trim().to_string();

        if response == "1" {
            add_password(key, user);
        } else if response == "2" {
            retrieve_password(key, user);
        } else if response == "b" {
            break;
        } else {
            println!("{response} is not an option");
        }
    }
}

fn add_password(key: &String, user: &String) {
    let mut account = String::new();
    println!("what is the name of the account associated with the password");
    std::io::stdin().read_line(&mut account).expect("TODO");
    account = account.trim().to_string();

    let mut password = String::new();
    println!("Type in the passowrd");
    std::io::stdin().read_line(&mut password).expect("TODO");
    password = password.trim().to_string();

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
    println!("what is the name of the account associated with the password");
    std::io::stdin().read_line(&mut account).expect("TODO");
    account = account.trim().to_string();

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
