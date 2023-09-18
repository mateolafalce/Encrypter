# Personal Encrypter 🔐

Change the **PATH** const in the `main.rs` file, with your path file.Then run `cargo run --release` and encrypt your critic content!

```rust
const PATH: &str = "/home/mateo/Escritorio/Passwords/passwords.txt";

fn main() {
    std::process::Command::new("clear").status().unwrap();
    let option: u8 = menu::menu();
    std::process::Command::new("clear").status().unwrap();
    match option {
        0 => encrypte_file::encrypte_file(PATH),
        1 => decrypt_file::decrypt_file(PATH),
        _ => (),
    }
}
```

## Dependencies

```toml
[dependencies]
magic-crypt = "3.1.12"
rpassword = "7.2.0"
termion = "2.0.1"
```

---

<div align="center">

!["screenshot"](./readme/screenshot.png)

</div>
