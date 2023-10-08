#[path = "../io/get_password.rs"]
mod get_password;

use magic_crypt::MagicCryptTrait;
use std::io::{Read, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use std::process;

pub fn encrypte_file(file_path: &str) {
    println!("You will overwrite {} file.", file_path);
    println!("Enter to continue");
    let mut user_input = std::io::stdout().into_raw_mode().unwrap();
    let input_in_console = std::io::stdin();

    for received_key in input_in_console.keys() {
        match received_key.unwrap() {
            Key::Char('\n') => {
                encrypte(file_path);
            },
            _ => {
                process::exit(1)
            }
        }
        user_input.flush().unwrap();
    }
}

pub fn encrypte(file_path: &str) {
    std::process::Command::new("clear").status().unwrap();
    let password = get_password::get_password("lock");
    match std::fs::File::open(file_path.clone()) {
        Ok(mut file) => {
            let mut critic_content: String = String::new();
            file.read_to_string(&mut critic_content).unwrap();
            let encryptor: magic_crypt::MagicCrypt256 =
                magic_crypt::new_magic_crypt!(password, 256);
            let str_to_base64: String = encryptor.encrypt_str_to_base64(&critic_content);
            std::fs::write(file_path.clone(), str_to_base64.clone()).unwrap();
            println!("{}", str_to_base64);
        }
        Err(error) => println!("error:{}", error),
    }
}
