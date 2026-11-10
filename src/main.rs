use std::{env::args, fs, io::Read};

use crate::{cartridge::Cartridge, gameboy::GameBoy};
mod cartridge;
mod gameboy;
mod mbc;
mod ppu;
mod sm83;

fn main() {
    let path = args()
        .nth(1)
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
    gb.run_n_cycles(2400000);
    let vram = gb.get_vram();
    println!("{vram:?}");
    for i in 0..384 {
        if i == 78 {
            println!("SHOULD BE N");
        }
        let tile = gb.get_tile(i);
        println!("{tile}\n");
    }
    let (tmap1, tmap2) = gb.get_tilemaps();
    println!("TILEMAP1:\n{tmap1}\n\n\n\n");
    println!("TILEMAP2:\n{tmap2}\n\n\n\n");
}
