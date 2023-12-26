/**************************************************************************
 *   utils.rs  --  This file is part of encrypter.                        *
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

use std::{path::Path, process};

pub fn check_existence(file_path: &str) -> std::io::Result<()> {
    if !Path::new(file_path).exists() {
        println!("File doesn't exist");
        process::exit(1);
    }
    Ok(())
}

pub fn verify_os() -> std::io::Result<()> {
    if !cfg!(target_os = "linux") {
        println!("Error, encrypter just suport \n -linux");
    }
    Ok(())
}
