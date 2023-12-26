/**************************************************************************
 *   decrypt_file.rs  --  This file is part of encrypter.                 *
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
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use std::io::Read;

pub fn decrypt_file(file_path: &str) -> std::io::Result<()> {
    let password: String = get_password("unlock");
    match std::fs::File::open(file_path) {
        Ok(mut file) => {
            let mut critic_content: String = String::new();
            file.read_to_string(&mut critic_content).unwrap();
            let decryptor: MagicCrypt256 = new_magic_crypt!(password, 256);
            match decryptor.decrypt_base64_to_string(&critic_content) {
                Ok(decrypt_content) => println!("{}", decrypt_content),
                Err(_) => println!("{}", critic_content),
            }
        }
        Err(error) => println!("error:{}", error),
    }
    Ok(())
}
