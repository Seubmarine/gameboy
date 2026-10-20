use crate::{cartridge::Cartridge, mbc::mbc_trait::MemoryBankController, sm83::SM83};

pub struct Bus {
    vram: [u8; 0x2000],
    wram: [[u8; 0x1000]; 8],
    mbc: Box<dyn MemoryBankController>,
}

impl Bus {
    pub fn new(mbc: Box<dyn MemoryBankController>) -> Self {
        Self {
            vram: [0; 0x2000],
            wram: [[0; 0x1000]; 8],
            mbc,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        println!("read addr: 0x{addr:X}");
        match addr {
            0x0000..=0x3FFF => self.mbc.read(addr),
            0x4000..=0x7FFF => self.mbc.read(addr),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.mbc.read(addr),
            0xC000..=0xCFFF => self.wram[0][(addr - 0xC000) as usize],
            0xD000..=0xDFFF => todo!("cgb: WRAM bank 1-7 (switchable)"),
            0xE000..=0xFDFF => todo!("echo ram (mirror 0xC000-0xDDFF)"),
            0xFE00..=0xFE9F => todo!("object attribute memory (oam)"),
            0xFEA0..=0xFEFF => todo!("not usable (prohibited)"),
            0xFF00..=0xFF7F => todo!("I/O Registers"),
            0xFF80..=0xFFFE => todo!("high ram"),
            0xFFFF..=0xFFFF => todo!("Interupt enable register"),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        println!("write addr: 0x{addr:X} value: {value} 0x{value:x}");
        match addr {
            0x0000..=0x3FFF => self.mbc.write(addr, value),
            0x4000..=0x7FFF => self.mbc.write(addr, value),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.mbc.write(addr, value),
            0xC000..=0xCFFF => self.wram[0][(addr - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.wram[1][(addr - 0xD000) as usize] = value, //todo!("cgb: WRAM bank 1-7 (switchable)")
            0xE000..=0xFDFF => self.write(addr - 0x2000, value),
            0xFE00..=0xFE9F => todo!("object attribute memory (oam)"),
            0xFEA0..=0xFEFF => todo!("not usable (prohibited)"),
            0xFF00..=0xFF7F => todo!("I/O Registers"),
            0xFF80..=0xFFFE => todo!("high ram"),
            0xFFFF..=0xFFFF => todo!("Interupt enable register"),
        };
    }
}

pub struct GameBoy {
    cpu: SM83,
    bus: Bus,
}

impl GameBoy {
    pub fn load(c: &Cartridge) -> GameBoy {
        let mbc = c.to_mbc();
        GameBoy {
            cpu: SM83::new(),
            bus: Bus::new(mbc),
        }
    }

    fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
}
