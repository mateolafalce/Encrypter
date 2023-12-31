/**************************************************************************
 *   show_menu_options.rs  --  This file is part of encrypter.            *
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

pub fn show_menu_options(option: &str) -> std::io::Result<()> {
    std::process::Command::new("clear").status()?;
    let mut encrypte: char = ' ';
    let mut decrypt: char = '*';
    if option == "lock selected" {
        encrypte = '*';
        decrypt = ' ';
    }
    println!("[{}] ENCRYPT THE FILE", encrypte);
    println!("[{}] DECRYPT THE FILE", decrypt);
    Ok(())
}
