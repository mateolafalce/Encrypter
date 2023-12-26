/**************************************************************************
 *   encrypte_file.rs  --  This file is part of encrypter.                *
 *                                                                        *
 *   Copyright (C) 2023 Mateo Lafalce                                     *
 *                                                                        *
 *   encrypter is free software: you can redistribute it and/or modify    *
 *   it under the terms of the GNU General Public License as published    *
 *   by the Free Software Foundation, either version 3 of the License,    *
 *   or (at your option) any later version.                               *
 *                                                                        *
 *   encrypter is distributed in the hope that it will be useful,         *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty          *
 *   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.              *
 *   See the GNU General Public License for more details.                 *
 *                                                                        *
 *   You should have received a copy of the GNU General Public License    *
 *   along with this program.  If not, see http://www.gnu.org/licenses/.  *
 *                                                                        *
 **************************************************************************/

use crate::io::get::get_password;
use k_board::{keyboard::Keyboard, keys::Keys};
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use std::io::Read;

pub fn encrypte_file(file_path: &str) -> std::io::Result<()> {
    std::process::Command::new("clear").status().unwrap();
    println!("You will overwrite {} file. Are you sure?", file_path);
    println!("Enter to continue...");
    for key in Keyboard::new() {
        if key == Keys::Enter {
            encrypte(file_path)?;
            break;
        }
    }
    Ok(())
}

pub fn encrypte(file_path: &str) -> std::io::Result<()> {
    std::process::Command::new("clear").status().unwrap();
    let password: String = get_password("lock");
    match std::fs::File::open(file_path) {
        Ok(mut file) => {
            let mut critic_content: String = String::new();
            file.read_to_string(&mut critic_content).unwrap();
            let encryptor: MagicCrypt256 = new_magic_crypt!(password, 256);
            let str_to_base64: String = encryptor.encrypt_str_to_base64(&critic_content);
            std::fs::write(file_path, str_to_base64.clone()).unwrap();
            println!("File successfully encrypted");
        }
        Err(error) => println!("error:{}", error),
    }
    Ok(())
}
