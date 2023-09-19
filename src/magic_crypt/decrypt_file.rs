#[path="../io/get_password.rs"]
mod get_password;

use magic_crypt::MagicCryptTrait;
use std::io::Read;

pub fn decrypt_file(file_path: &str){
	std::process::Command::new("clear").status().unwrap();
	let password: String = get_password::get_password("unlock");
	match std::fs::File::open(file_path.clone()) {
		Ok(mut file) => {
			let mut critic_content: String = String::new();
			file.read_to_string(&mut critic_content).unwrap();
			let decryptor: magic_crypt::MagicCrypt256 = magic_crypt::new_magic_crypt!(password, 256);
			match decryptor.decrypt_base64_to_string(&critic_content) {
				Ok(decrypt_content) => println!("{}", decrypt_content),
				Err(_) => println!("{}", critic_content),
			}
		},
		Err(error) => println!("error:{}", error),
	}
}