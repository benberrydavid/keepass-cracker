use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};
use rpassword::read_password;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct KeePassEntry {
    title: String,
    username: String,
    password: String,
}

fn main() {
    let matches = App::new("KeePass Cracker")
        .version("1.0")
        .author("benberrydavid")
        .about("Cracks KeePass safes")
        .arg(Arg::new("file")
            .about("KeePass file to crack")
            .required(true)
            .index(1))
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let password = read_password().unwrap();
    let entries = crack_keepass(file_path, &password);
    for entry in entries {
        println!("Title: {}, Username: {}, Password: {}", entry.title, entry.username, entry.password);
    }
}

fn crack_keepass(file_path: &str, password: &str) -> Vec<KeePassEntry> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Unable to read file");
    decrypt_data(&data, password)
}

fn decrypt_data(data: &[u8], password: &str) -> Vec<KeePassEntry> {
    let decrypted_data = aes_decrypt(data, password);
    serde_json::from_slice(&decrypted_data).expect("Failed to parse decrypted data")
}

fn aes_decrypt(data: &[u8], password: &str) -> Vec<u8> {
    let key = generate_key(password);
    let cipher = aes::Aes256::new(&key);
    let mut buffer = data.to_vec();
    cipher.decrypt(&mut buffer);
    buffer
}

fn generate_key(password: &str) -> Vec<u8> {
    let mut key = vec![0u8; 32];
    key.copy_from_slice(&password.as_bytes()[..32]);
    key
}