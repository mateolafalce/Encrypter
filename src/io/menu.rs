#[path="./show_menu_options.rs"]
mod show_menu_options;

use termion::{
    raw::IntoRawMode,
    input::TermRead,
    event::Key,
};
use std::io::Write;

pub fn menu() -> u8 {
    show_menu_options::show_menu_options(false);
    let mut user_input = std::io::stdout().into_raw_mode().unwrap();
    let input_in_console: std::io::Stdin = std::io::stdin();
    let mut encrypte_decrypt_option: u8 = 0;
    for received_key in input_in_console.keys() {
        match received_key.unwrap() {
            Key::Up => {
                show_menu_options::show_menu_options(false);
                encrypte_decrypt_option = 0;
            }
            Key::Down => {
                show_menu_options::show_menu_options(true);
                encrypte_decrypt_option = 1;
            }
            Key::Char('\n') => {
                break;
            }
            _ => {},
        }
        user_input.flush().unwrap();
    }
    encrypte_decrypt_option
}
