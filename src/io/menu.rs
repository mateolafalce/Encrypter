use termion::{
    color,
    raw::IntoRawMode,
    input::TermRead
};
use std::io::Write;

pub fn menu() -> u8 {
    std::process::Command::new("clear").status().unwrap();
    println!("{}[*] ENCRYPTE THE FILE", color::Fg(color::LightWhite));
    println!("{}[ ] DECRYPT THE FILE", color::Fg(color::White));
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdin: std::io::Stdin = std::io::stdin();
    let mut option: u8 = 0;
    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Up => {
                std::process::Command::new("clear").status().unwrap();
                println!("{}[*] ENCRYPTE THE FILE\r", color::Fg(color::LightWhite));
                println!("{}[ ] DECRYPT THE FILE\r", color::Fg(color::White));
                option = 0;
            }
            termion::event::Key::Down => {
                std::process::Command::new("clear").status().unwrap();
                println!("{}[ ] ENCRYPTE THE FILE\r",color::Fg(color::White));
                println!("{}[*] DECRYPT THE FILE\r",color::Fg(color::LightWhite));
                option = 1;
            }
            termion::event::Key::Char('\n') => {
                break;
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    std::process::Command::new("clear").status().unwrap();
    option
}
