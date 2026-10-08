use crate::mbc::mbc_trait::{self, MemoryBankController};
//DOC: https://gbdev.io/pandocs/nombc.html
struct RomOnly<'rom> {
    rom: &'rom [u8],
}

impl<'rom> RomOnly<'rom> {
    fn new(rom: &'rom [u8]) -> Self {
        Self { rom }
    }
}

impl MemoryBankController for RomOnly<'_> {
    fn write(&mut self, _addr: u16, _value: u8) {
        panic!("Can't use write on rom in a rom only")
    }

    fn read(&self, addr: u16) -> u8 {
        if addr > 0x7FFF {
            unreachable!("Rom only can't read outside this range")
        }
        self.rom[addr as usize]
    }
}
