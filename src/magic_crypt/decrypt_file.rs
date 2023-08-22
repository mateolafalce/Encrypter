#[path="../io/get_password.rs"]
mod get_password;
use magic_crypt::MagicCryptTrait;
use std::io::Read;

pub fn decrypt_file(path: &str){
	std::process::Command::new("clear").status().unwrap();
	//Get unlock password
	let password = get_password::get_password("unlock");
	match std::fs::File::open(path.clone()) {
		Ok(mut file) => {
			//Decrypt file selected
			let mut content: String = String::new();
			file.read_to_string(&mut content).unwrap();
			let mc: magic_crypt::MagicCrypt256 = magic_crypt::new_magic_crypt!(password, 256);
			//Show decrypt file
			std::process::Command::new("clear").status().unwrap();
			//println!("{}", mc.decrypt_base64_to_string(&content).unwrap());
			match mc.decrypt_base64_to_string(&content) {
				Ok(passwords) => println!("{}",passwords),
				Err(_) => println!("{}", content),
			}
		},
		Err(e) => println!("File doesn`t exist, e:{}", e),
	}
}