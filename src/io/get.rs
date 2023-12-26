/**************************************************************************
 *   get.rs  --  This file is part of encrypter.                          *
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

use rpassword::read_password;

pub fn get_password(lock_unlock: &str) -> String {
    println!("Enter the password to {} of your file", lock_unlock);
    read_password().unwrap()
}

pub fn get_path() -> String {
    println!("Enter the path of your file to encrypte");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
