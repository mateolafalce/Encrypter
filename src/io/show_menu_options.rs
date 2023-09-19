pub fn show_menu_options(option: bool){
	std::process::Command::new("clear").status().unwrap();
	let mut selection_encrypte: char = ' ';
	let mut selection_decrypt: char = '*';
	if !option {
		selection_encrypte = '*';
		selection_decrypt = ' ';
	}
    println!("[{}] ENCRYPTE THE FILE",  selection_encrypte);
    println!("[{}] DECRYPT THE FILE",  selection_decrypt);
}