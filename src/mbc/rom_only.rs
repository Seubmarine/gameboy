use crate::mbc::mbc_trait::MemoryBankController;
//DOC: https://gbdev.io/pandocs/nombc.html
pub struct RomOnly {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl RomOnly {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom, ram: vec![0; crate::Cartridge::RAM_BANK_SIZE] }
    }
}

impl MemoryBankController for RomOnly {

    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0xA000..=0xBFFF => self.ram[addr as usize - 0xA000],
            _ => unreachable!()
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => (),
            0xA000..=0xBFFF => self.ram[addr as usize - 0xA000] = value,
            _ => unreachable!()
        }
    }
}
