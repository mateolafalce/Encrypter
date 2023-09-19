#[path="./magic_crypt/encrypte_file.rs"]
mod encrypte_file;
#[path="./magic_crypt/decrypt_file.rs"]
mod decrypt_file;
#[path="./io/check_existence.rs"]
mod check_existence;
#[path="./io/menu.rs"]
mod menu;

const PATH: &str = "";

fn main(){
    check_existence::check_existence(PATH);
    let encrypte_decrypt_option: u8 = menu::menu();
    match encrypte_decrypt_option {
        0 => encrypte_file::encrypte_file(PATH),
        1 => decrypt_file::decrypt_file(PATH),
        _ => (),
    }
}
