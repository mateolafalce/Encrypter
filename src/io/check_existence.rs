use std::{
	path::Path,
	process
};

pub fn check_existence(file_path: &str){
	if !Path::new(file_path).exists() {
		println!("File doesn't exist");
		process::exit(1);
	}
}