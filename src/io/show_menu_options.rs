pub fn show_menu_options(option: bool){
	std::process::Command::new("clear").status().unwrap();
	let mut encrypte: char = ' ';
	let mut decrypt: char = '*';
	if !option {
		encrypte = '*';
		decrypt = ' ';
	}
    println!("[{}] ENCRYPTE THE FILE", encrypte);
    println!("[{}] DECRYPT THE FILE", decrypt);
}