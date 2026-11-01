use crate::mbc::mbc_trait::MemoryBankController;
//DOC: https://gbdev.io/pandocs/MBC1.html#mbc1

#[derive(Debug, PartialEq)]
enum BankingMode {
    Simple,   //ROM Banking Mode (up to 8KByte RAM, 2MByte ROM) (default)
    Advanced, //RAM Banking Mode (up to 32KByte RAM, 512KByte ROM)
}

pub struct MBC1 {
    rom: Vec<u8>,
    _rom_ram_bank_register: u8, //5bit register
    ram_enable: bool,
    rom_size: usize,
    ram: Vec<u8>,
    ram_size: usize,
    banking_mode: BankingMode,
}

impl MBC1 {
    const RAM_BANK_SIZE: usize = 0x2000;
    const ROM_BANK_SIZE: usize = 0x4000;
    pub fn new(rom: Vec<u8>, rom_size: usize, ram_size: usize) -> Self {
        Self {
            rom,
            _rom_ram_bank_register: 0,
            ram_enable: false,
            rom_size,
            ram_size,
            ram: vec![0; Self::RAM_BANK_SIZE * ram_size as usize],
            banking_mode: BankingMode::Simple,
        }
    }

    fn get_rom_ram_upper2(&self) -> usize {
        (self._rom_ram_bank_register >> 5) as usize
    }

    fn get_ram_bank_number(&self) -> usize {
        assert!(self.banking_mode == BankingMode::Simple);
        self.get_rom_ram_upper2()
    }

    fn set_ram_rom_upper2(&mut self, value: u8) {
        self._rom_ram_bank_register = (value << 5) | (self._rom_ram_bank_register & 0b11111);
    }

    //set bit 6 and 7 of rom_ram
    fn set_ram_bank(&mut self, value: u8) {
        assert_eq!(self.banking_mode, BankingMode::Advanced);
        self.set_ram_rom_upper2(value);
    }

    //set bit 6 and 7 of rom_ram
    fn set_rom_bank_extra(&mut self, value: u8) {
        assert_eq!(self.banking_mode, BankingMode::Simple);
        self.set_ram_rom_upper2(value);
    }

    //set lowest 5 bit of rom_ram
    fn set_rom_bank(&mut self, value: u8) {
        self._rom_ram_bank_register = (self._rom_ram_bank_register & 0b1100000) | (value & 0b11111);
    }

    fn get_rom_bank_mask(&self) -> u8 {
        let rom_bank_count: usize = self.rom_size / Self::ROM_BANK_SIZE;
        let mask = rom_bank_count.next_power_of_two() - 1;
        mask as u8
    }

    //https://gbdev.io/pandocs/MBC1.html#20003fff--rom-bank-number-write-only
    //MBC1 cartridge select the first bank if the bank number is 0
    fn get_rom_bank_number(&self) -> u8 {
        let mask = self.get_rom_bank_mask();
        let rom_bank_current = self._rom_ram_bank_register;
        if rom_bank_current & 0b11111 == 0 {
            return rom_bank_current + 1 & mask;
        }
        rom_bank_current & mask
    }

    fn rom_bank_mask() {}
}

impl MemoryBankController for MBC1 {
    //DOC: https://gbdev.io/pandocs/MBC1.html#memory
    fn read(&self, addr: u16) -> u8 {
        match addr {
            //first 16Kb of the rom
            0x000..=0x3FFF => {
                match self.banking_mode {
                    BankingMode::Simple => self.rom[addr as usize], //bank 0
                    BankingMode::Advanced => self.rom[(self.get_rom_ram_upper2() << 5) * 0x4000], //access either of (0x00, 0x20, 0x40, 0x60) in advanced mode
                }
            }
            //index into the rom bank
            0x4000..=0x7FFF => {
                let bank_index = Self::ROM_BANK_SIZE * self.get_rom_bank_number() as usize;
                let index = (addr - 0x4000) as usize;
                self.rom[bank_index + index]
            }
            //index into the ram bank
            0xA000..=0xBFFF => {
                if !self.ram_enable {
                    return 0xFF;
                }

                let bank_index = Self::RAM_BANK_SIZE
                    * match self.banking_mode {
                        BankingMode::Simple => 0,
                        BankingMode::Advanced => self.get_ram_bank_number(),
                    };

                let index = (addr - 0xA000) as usize;
                self.ram[bank_index + index]
            }
            _ => unreachable!("read at addr {addr:#X}"),
        }
    }

    //DOC: https://gbdev.io/pandocs/MBC1.html#registers
    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            //Ram enable/disable to read and write to ram
            0x0000..=0x1FFF => {
                if value & 0b1111 == 0xA {
                    self.ram_enable = true
                } else {
                    self.ram_enable = false
                }
            }
            //rom bank number selection
            0x2000..=0x3FFF => {
                match self.banking_mode {
                    BankingMode::Simple => {
                        self.set_rom_bank(value);
                    }
                    BankingMode::Advanced => todo!(),
                }
                //the register is a 5 bit register
            }
            //select ram or upper bits rom bank number
            //in advanced banking mode this value change the ram bank number (from 0x00-0x03)
            //in simple banking mode this value changes the upper two bits of the rom bank
            //(this allow accessing a bigger rom space at the cost of less ram space)
            0x4000..=0x5FFF => match self.banking_mode {
                BankingMode::Simple => {
                    self.set_rom_bank_extra(value);
                }
                BankingMode::Advanced => {
                    self.set_ram_bank(value);
                }
            },
            //modify banking mode
            0x6000..=0x7FFF => {
                if addr & 0b1 == 1 {
                    self.banking_mode = BankingMode::Advanced;
                } else {
                    self.banking_mode = BankingMode::Simple;
                }
            }
            //index into the ram bank
            0xA000..=0xBFFF => {
                if !self.ram_enable {
                    return;
                }

                let bank_index = Self::RAM_BANK_SIZE
                    * match self.banking_mode {
                        BankingMode::Simple => 0,
                        BankingMode::Advanced => self.get_ram_bank_number(),
                    };

                let index = (addr - 0xA000) as usize;
                self.ram[bank_index + index] = value
            }
            _ => unreachable!("write in mbc1 at address: {addr:#X}"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{cartridge::Cartridge, gameboy::GameBoy};
    use std::fs::File;
    use std::io::Read;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn mbc1_setup() -> MBC1 {
        const KB: usize = 1024;
        const MIB: usize = KB * 1024;
        let mbc = MBC1::new(vec![], 2 * MIB, 32 * KB);
        mbc
    }

    #[test]
    fn test_ram_enable_disable() {
        let mut mbc = mbc1_setup();

        mbc.write(0x0000, 0xA);
        assert!(
            mbc.ram_enable == true,
            "write at region 0x0000 - 0x1FFF with 0xA should enable ram"
        );
        mbc.write(0x1FFF, 0x10);
        assert!(
            mbc.ram_enable == false,
            "write at region 0x0000 - 0x1FFF with not 0xA should disable ram"
        );
        mbc.write(0x0123, 0xFA);
        assert!(
            mbc.ram_enable == true,
            "write at region 0x0000 - 0x1FFF with not lowest bits set to 0xA should enable ram"
        );
    }

    #[test]
    fn test_rom_256kb_map_bank_0() {
        let mut mbc = MBC1::new(vec![], 1024 * 256, 0);
        mbc.write(0x2000, 0b10000);
        //the register check if it's value is set to 0 and return 1 if so (bank 0 shouldn't be selectable)
        //but passing the value 0b10000 disable this check since it's not 0b00000
        //256KiB / Rom bank size(16KiB) == 16 Bank, this map to a value between 0x0 to 0xF, it's represented as a mask of 0b1111
        //the register should return a the value 0b10000 & 0b011111 == 0b00000 which make the region 0 accessible
        assert!(
            mbc.get_rom_bank_number() == 0,
            "Mapping rom bank number 0 is possible when rom size is equal or lower than 256KiB"
        )
        //This is normal behavior and should be correct for cartridge under or equal to 256Kib
    }

    #[test]
    fn test_rom_bank_selection_512kib() {
        //32 * 0x4000 == 512KiB
        let mut mbc = MBC1::new(vec![], 32 * 0x4000, 0);

        for i in 0..32 {
            mbc.write(0x2000, i);
            if i == 0 {
                assert_eq!(
                    mbc.get_rom_bank_number(),
                    1,
                    "bank rom number 0 should be set to 1"
                );
            } else {
                assert_eq!(
                    mbc.get_rom_bank_number() as u8,
                    i,
                    "bank rom should be set as the same number"
                )
            }
        }
    }

    #[test]
    fn test_rom_bank_selection_2mib() {
        //prepare rom
        let mut rom: Vec<u8> = vec![u8::MAX; 1024 * 2048]; //2MiB

        for i in 0..128 {
            //128 Bank of size 16384 (0x4000)
            rom[i * 0x4000] = i as u8;
        }

        let rom_size = rom.len();
        let mut mbc = MBC1::new(rom, rom_size, 0);

        for i in 0..128 {
            //low bit set
            mbc.write(0x2000, i);
            //high bit set
            mbc.write(0x4000, i >> 5);

            let byte = mbc.read(0x4000);
            if i & 0b11111 == 0 {
                assert_eq!(
                    byte,
                    (i + 1),
                    "Bank 0x00 0x20 0x40 0x60 should point to their next bank"
                );
            } else {
                assert_eq!(byte, i, "bank rom should be set as the same number")
            }
        }

        mbc.write(0x7FFF, 1); //Advanced banking mode
        assert_eq!(
            mbc.banking_mode,
            BankingMode::Advanced,
            "Write from 0x6000 - 0x7FFF with 1 switch banking_mode"
        );

        for i in [0x00, 0x20, 0x40, 0x60] {
            mbc.write(0x4000, i >> 5);
            let byte = mbc.read(0x0000); //bank 0 is switchable in advanced mode
            assert_eq!(
                i, byte,
                "bank 0x00, 0x20, 0x40, 0x60 are accessible in region 0x0000 - 0x3FFF in advanced mode"
            )
        }
    }

    fn gameboy_load(path: &str) -> GameBoy {
        let mut file = File::open(path).unwrap();
        let mut rom = Vec::new();
        file.read_to_end(&mut rom).unwrap();
        let c = Cartridge::load(&rom).unwrap();
        let mut g = GameBoy::load(&c);
        g.test_mode_mooneye();
        g
    }

    const MAX_STEPS: usize = 10000000;
    fn gameboy_run_test(path: &str) {
        let mut gb = gameboy_load(path);
        for _ in 0..MAX_STEPS {
            gb.step();
            match crate::sm83::tests::test_check_mooneye(&mut gb.cpu, &mut gb.bus) {
                Some(true) => return,
                Some(false) => panic!("ROM reported failure at pc={:#06x}", gb.cpu.pc),
                None => (),
            }
        }
        panic!("timeout, last pc={:#06x}", gb.cpu.pc);
    }

    #[test]
    fn test_rom_512kb() {
        let g = gameboy_run_test("tests/test-roms/mooneye/mbc1/rom_512kb.gb");
    }

    #[test]
    fn test_ram_64kb() {
        let g = gameboy_run_test("tests/test-roms/mooneye/mbc1/ram_64kb.gb");
    }
}
