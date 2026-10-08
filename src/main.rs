use std::{env::args, fs, io::Read};

use crate::cartridge::Cartridge;
mod cartridge;
mod mbc;
mod sm83;
fn main() {
    let rom_folder = fs::read_dir("rom")
        .expect("Missing folder rom")
        .next()
        .expect("Need a file in the rom folder")
        .unwrap();
    let mut file = fs::File::open(rom_folder.path()).unwrap();

    let mut file_bytes = Vec::new();
    // let c = Cartridge::new();
    // println!("{c:?}");
    file.read_to_end(&mut file_bytes).unwrap();

    let c = Cartridge::load(&file_bytes).unwrap();
    for w in &c.warnings {
        println!("Warning!: {w:?}");
    }

    println!("{c}");
}
