#[path="../io/get_password.rs"]
mod get_password;

use magic_crypt::MagicCryptTrait;
use std::io::Read;

pub fn encrypte_file(file_path: &str){
	let password = get_password::get_password("lock");
	match std::fs::File::open(file_path.clone()) {
		Ok(mut file) => {
			let mut critic_content: String = String::new();
			file.read_to_string(&mut critic_content).unwrap();
			let encryptor: magic_crypt::MagicCrypt256 = magic_crypt::new_magic_crypt!(password, 256);
			let str_to_base64: String = encryptor.encrypt_str_to_base64(&critic_content);
			std::fs::write(file_path.clone(), str_to_base64.clone()).unwrap();
			println!("{}", str_to_base64);
		},
		Err(error) => println!("error:{}", error),
	}
}