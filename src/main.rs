use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, stdin, BufRead, BufReader};

fn main() {
    println!("Welcome to Cruz Password Manager");
    println!("press 1 to create an account");
    println!("press 2 to login to an account");
    let mut response = String::new();
    stdin().read_line(&mut response).unwrap();
    response = response.trim().to_string();
    let mut key = String::new();
    loop {
        if response == "1" {
            signup(&mut key);
        } else if response == "2" {
            login();
        } else {
            panic!();
        }
    }
}

fn signup(key: &mut Sting) -> Result<(), ()> {
    let mut username = String::new();
    let database = "database.db";
    println!("Please input a username");
    stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();

    println!("Please input a password");
    let mut password = String::new();
    let mut password_hasher = DefaultHasher::new();
    stdin().read_line(&mut password).unwrap();
    password = password.trim().to_string();
    password.hash(&mut password_hasher);
    password = password_hasher.finish().to_string();
    println!(
        "your username is {} and your password is {}",
        username, password
    );

    let user = username + " " + password.as_str();

    std::fs::write(database, &user).unwrap_or({
        std::fs::File::create(database).unwrap();
        std::fs::write(database, &user).unwrap();
    });
    Ok(key)
}

fn login() -> io::Result<()> {
    let mut username = String::new();
    let mut password = String::new();

    println!("input your username");
    stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();
    println!("input your password");

    stdin().read_line(&mut password).unwrap();
    password = password.trim().to_string();
    let key = getSymmetricKey(password);
    let mut password_hasher = DefaultHasher::new();
    password.hash(&mut password_hasher);
    password = password_hasher.finish().to_string();

    let database = "database.db";
    let file = File::open(database)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let u: Vec<_> = line.split(" ").collect();
        if username == u[0] && password == u[1] {
            println!("Welcome back {username}");
        }
    }

    std::fs::read(database).expect("should be able to read {database}");
    Ok(key)
}

fn 
