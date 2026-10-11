use std::{env::args, fs, io::Read};

use crate::{cartridge::Cartridge, gameboy::GameBoy};
mod cartridge;
mod gameboy;
mod mbc;
mod sm83;

fn main() {
    let path = args()
        .next()
        .unwrap_or("tests/test-roms/mooneye/mbc1/rom_512kb.gb".to_string());
    let mut file = fs::File::open(path).unwrap();

    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes).unwrap();

    let c = Cartridge::load(&file_bytes).unwrap();
    for w in &c.warnings {
        println!("Warning!: {w:?}");
    }

    println!("{c}");

    let mut gb = GameBoy::load(&c);
    gb.run();
}
