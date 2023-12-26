# Encrypter 🔐

Change the **PATH** const in the `main.rs` file, with your path file. Then run `cargo run --release` and encrypt your critic content!

```rust
mod io;
mod magic_crypt;
mod utils;

use crate::{
    io::{get::get_path, menu::menu},
    magic_crypt::{decrypt_file::decrypt_file, encrypte_file::encrypte_file},
    utils::{check_existence, verify_os},
};

fn main() -> std::io::Result<()> {
    verify_os()?;
    let path: &str = &get_path();
    check_existence(path)?;
    let encrypte_decrypt_option: u8 = menu();
    match encrypte_decrypt_option {
        0 => encrypte_file(path),
        1 => decrypt_file(path),
        _ => std::process::exit(1),
    }?;
    Ok(())
}
```

---

<div align="center">

!["screenshot"](./readme/screenshot.png)

</div>
