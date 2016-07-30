use std::{fs, io};
use std::process::Command;

fn main() {
    run().unwrap();
}

fn run() -> io::Result<()> {
    try!(fs::create_dir_all(env!("OUT_DIR")));

    assert!(try!(Command::new("nasm")
            .args(&["-f", "elf64"])
            .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/src/boot.S"))
            .args(&["-o", concat!(env!("OUT_DIR"), "/boot.o")])
            .status())
        .success());

    assert!(try!(Command::new("ar")
            .arg("crus")
            .arg(concat!(env!("OUT_DIR"), "/libboot.a"))
            .arg(concat!(env!("OUT_DIR"), "/boot.o"))
            .status())
        .success());

    println!(concat!("cargo:rustc-link-search=", env!("OUT_DIR")));
    println!("cargo:rustc-link-lib=static=boot");

    Ok(())
}
