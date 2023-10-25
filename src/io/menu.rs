#[path = "./show_menu_options.rs"]
mod show_menu_options;

use k_board::{Keyboard, Keys};

pub fn menu() -> u8 {
    show_menu_options::show_menu_options("lock selected");
    let mut encrypt_decrypt_option: u8 = 0;
    for key in Keyboard::new() {
        match key {
            Keys::Up => {
                show_menu_options::show_menu_options("lock selected");
                encrypt_decrypt_option = 0;
            }
            Keys::Down => {
                show_menu_options::show_menu_options("unlock selected");
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
