#[path="./magic_crypt/encrypte_file.rs"]
mod encrypte_file;
#[path="./magic_crypt/decrypt_file.rs"]
mod decrypt_file;
#[path="./io/menu.rs"]
mod menu;

const PATH: &str = "/path/passwords.txt";

fn main(){
    let option: u8 = menu::menu();
    match option {
        0 => encrypte_file::encrypte_file(PATH),
        1 => decrypt_file::decrypt_file(PATH),
        _ => (),
    }
}
