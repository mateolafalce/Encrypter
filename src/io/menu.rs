/**************************************************************************
 *   menu.rs  --  This file is part of encrypter.                         *
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

use crate::io::show_menu_options::show_menu_options;
use k_board::{keyboard::Keyboard, keys::Keys};

pub fn menu() -> u8 {
    show_menu_options("lock selected").unwrap();
    let mut encrypt_decrypt_option: u8 = 0;
    for key in Keyboard::new() {
        match key {
            Keys::Up => {
                show_menu_options("lock selected").unwrap();
                encrypt_decrypt_option = 0;
            }
            Keys::Down => {
                show_menu_options("unlock selected").unwrap();
                encrypt_decrypt_option = 1;
            }
            Keys::Enter => {
                break;
            }
            _ => {}
        }
    }
    std::process::Command::new("clear").status().unwrap();
    encrypt_decrypt_option
}
