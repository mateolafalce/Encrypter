/**************************************************************************
 *   main.rs  --  This file is part of encrypter.                         *
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

mod io;
mod magic_crypt;
mod utils;

use crate::{
    io::{get::get_path, menu::menu},
    magic_crypt::{decrypt_file::decrypt_file, encrypte_file::encrypte_file},
    utils::{check_existence, verify_os},
};

fn main() -> std::io::Result<()> {
    verify_os()?;
    let path: &str = &get_path();
    check_existence(path)?;
    let encrypte_decrypt_option: u8 = menu();
    match encrypte_decrypt_option {
        0 => encrypte_file(path),
        1 => decrypt_file(path),
        _ => std::process::exit(1),
    }?;
    Ok(())
}
