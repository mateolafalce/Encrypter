#[path="../io/get_password.rs"]
mod get_password;
use magic_crypt::MagicCryptTrait;
use std::io::Read;

pub fn encrypte_file(path: &str){
	std::process::Command::new("clear").status().unwrap();
	//Get lock password
	let password = get_password::get_password("lock");
	match std::fs::File::open(path.clone()) {
		Ok(mut file) => {
			//Encrypt the file select
			let mut content: String = String::new();
			file.read_to_string(&mut content).unwrap();
			let mc: magic_crypt::MagicCrypt256 = magic_crypt::new_magic_crypt!(password, 256);
			let base64: String = mc.encrypt_str_to_base64(&content);
			//Write file
			std::fs::write(path.clone(), base64.clone()).expect("Unable to write file");
			std::process::Command::new("clear").status().unwrap();
			//Show the file content
			println!("{}", base64);
		},
		Err(e) => println!("File doesn`t exist, e:{}", e),
	}
}