use termion::color;

pub fn show_menu_options(option: &str) {
    std::process::Command::new("clear").status().unwrap();
    let mut encrypte: char = ' ';
    let mut decrypt: char = '*';
    if option == "lock selected" {
        encrypte = '*';
        decrypt = ' ';
    }
    println!(
        "{}[{}] ENCRYPT THE FILE\r",
        color::Fg(color::LightWhite),
        encrypte
    );
    println!("[{}] DECRYPT THE FILE\r", decrypt);
}
