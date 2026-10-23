use crate::{cartridge::Cartridge, mbc::mbc_trait::MemoryBankController, sm83::SM83};

pub struct Bus {
    vram: [u8; 0x2000],
    wram: [[u8; 0x1000]; 8],
    wram_bank_current: usize,
    mbc: Box<dyn MemoryBankController>,
    hardware_registers: HardwareRegisters,
    hram: [u8; 127],
}

impl Bus {
    pub fn new(mbc: Box<dyn MemoryBankController>) -> Self {
        Self {
            vram: [0; 0x2000],
            wram: [[0; 0x1000]; 8],
            wram_bank_current: 1,
            mbc,
            hardware_registers: Default::default(),
            hram: [0; 127],
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        // println!("read addr: 0x{addr:X}");
        match addr {
            0x0000..=0x3FFF => self.mbc.read(addr),
            0x4000..=0x7FFF => self.mbc.read(addr),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.mbc.read(addr),
            0xC000..=0xCFFF => self.wram[0][(addr - 0xC000) as usize],
            0xD000..=0xDFFF => self.wram[self.wram_bank_current][(addr - 0xD000) as usize], //todo!("cgb: WRAM bank 1-7 (switchable)"),
            0xE000..=0xFDFF => self.read(addr - 0x2000), //todo!("echo ram (mirror 0xC000-0xDDFF)"),
            0xFE00..=0xFE9F => todo!("object attribute memory (oam)"),
            0xFEA0..=0xFEFF => todo!("not usable (prohibited)"),
            0xFF00..=0xFF7F => self.hardware_registers.read_hardware_register(addr),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF..=0xFFFF => self.hardware_registers.read_hardware_register(addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        // println!("write addr: 0x{addr:X} value: {value} 0x{value:x}");
        match addr {
            0x0000..=0x3FFF => self.mbc.write(addr, value),
            0x4000..=0x7FFF => self.mbc.write(addr, value),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.mbc.write(addr, value),
            0xC000..=0xCFFF => self.wram[0][(addr - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.wram[self.wram_bank_current][(addr - 0xD000) as usize] = value, //todo!("cgb: WRAM bank 1-7 (switchable)")
            0xE000..=0xFDFF => self.write(addr - 0x2000, value),
            0xFE00..=0xFE9F => todo!("object attribute memory (oam)"),
            0xFEA0..=0xFEFF => todo!("not usable (prohibited)"),
            0xFF00..=0xFF7F => self.hardware_registers.write_hardware_register(addr, value),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF..=0xFFFF => self.hardware_registers.write_hardware_register(addr, value),
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

enum HardwareRegisterName {
    P1,      //0xFF00
    SB,      //0xFF01
    SC,      //0xFF02
    DIV,     //0xFF04
    TIMA,    //0xFF05
    TMA,     //0xFF06
    TAC,     //0xFF07
    IF,      //0xFF0F
    NR10,    //0xFF10
    NR11,    //0xFF11
    NR12,    //0xFF12
    NR13,    //0xFF13
    NR14,    //0xFF14
    NR21,    //0xFF16
    NR22,    //0xFF17
    NR23,    //0xFF18
    NR24,    //0xFF19
    NR30,    //0xFF1A
    NR31,    //0xFF1B
    NR32,    //0xFF1C
    NR33,    //0xFF1D
    NR34,    //0xFF1E
    NR41,    //0xFF20
    NR42,    //0xFF21
    NR43,    //0xFF22
    NR44,    //0xFF23
    NR50,    //0xFF24
    NR51,    //0xFF25
    NR52,    //0xFF26
    WaveRAM, //0xFF30
    LCDC,    //0xFF40
    STAT,    //0xFF41
    SCY,     //0xFF42
    SCX,     //0xFF43
    LY,      //0xFF44
    LYC,     //0xFF45
    DMA,     //0xFF46
    BGP,     //0xFF47
    OBP0,    //0xFF48
    OBP1,    //0xFF49
    WY,      //0xFF4A
    WX,      //0xFF4B
    KEY0,    //0xFF4C
    KEY1,    //0xFF4D
    VBK,     //0xFF4F
    BANK,    //0xFF50
    HDMA1,   //0xFF51
    HDMA2,   //0xFF52
    HDMA3,   //0xFF53
    HDMA4,   //0xFF54
    HDMA5,   //0xFF55
    RP,      //0xFF56
    BCPS,    //0xFF68
    BCPD,    //0xFF69
    OCPS,    //0xFF6A
    OCPD,    //0xFF6B
    OPRI,    //0xFF6C
    SVBK,    //0xFF70
    PCM12,   //0xFF76
    PCM34,   //0xFF77
    IE,      //0xFFFF
}

#[derive(Debug, Default)]
pub struct HardwareRegisters {
    p1: u8,             //0xFF00
    sb: u8,             //0xFF01
    sc: u8,             //0xFF02
    div: u8,            //0xFF04
    tima: u8,           //0xFF05
    tma: u8,            //0xFF06
    tac: u8,            //0xFF07
    if_: u8,            //0xFF0F
    nr10: u8,           //0xFF10
    nr11: u8,           //0xFF11
    nr12: u8,           //0xFF12
    nr13: u8,           //0xFF13
    nr14: u8,           //0xFF14
    nr21: u8,           //0xFF16
    nr22: u8,           //0xFF17
    nr23: u8,           //0xFF18
    nr24: u8,           //0xFF19
    nr30: u8,           //0xFF1A
    nr31: u8,           //0xFF1B
    nr32: u8,           //0xFF1C
    nr33: u8,           //0xFF1D
    nr34: u8,           //0xFF1E
    nr41: u8,           //0xFF20
    nr42: u8,           //0xFF21
    nr43: u8,           //0xFF22
    nr44: u8,           //0xFF23
    nr50: u8,           //0xFF24
    nr51: u8,           //0xFF25
    nr52: u8,           //0xFF26
    wave_ram: [u8; 16], //0xFF30-0xFF3F
    lcdc: u8,           //0xFF40
    stat: u8,           //0xFF41
    scy: u8,            //0xFF42
    scx: u8,            //0xFF43
    ly: u8,             //0xFF44
    lyc: u8,            //0xFF45
    dma: u8,            //0xFF46
    bgp: u8,            //0xFF47
    obp0: u8,           //0xFF48
    obp1: u8,           //0xFF49
    wy: u8,             //0xFF4A
    wx: u8,             //0xFF4B
    key0: u8,           //0xFF4C
    key1: u8,           //0xFF4D
    vbk: u8,            //0xFF4F
    bank: u8,           //0xFF50
    hdma1: u8,          //0xFF51
    hdma2: u8,          //0xFF52
    hdma3: u8,          //0xFF53
    hdma4: u8,          //0xFF54
    hdma5: u8,          //0xFF55
    rp: u8,             //0xFF56
    bcps: u8,           //0xFF68
    bcpd: u8,           //0xFF69
    ocps: u8,           //0xFF6A
    ocpd: u8,           //0xFF6B
    opri: u8,           //0xFF6C
    svbk: u8,           //0xFF70
    pcm12: u8,          //0xFF76
    pcm34: u8,          //0xFF77
    ie: u8,             //0xFFFF
}

impl HardwareRegisters {
    fn get_hardware_register(&mut self, addr: u16) -> Option<&mut u8> {
        let data: &mut u8 = match addr {
            0xFF00 => &mut self.p1,
            0xFF01 => &mut self.sb,
            0xFF02 => &mut self.sc,
            0xFF04 => &mut self.div,
            0xFF05 => &mut self.tima,
            0xFF06 => &mut self.tma,
            0xFF07 => &mut self.tac,
            0xFF0F => &mut self.if_,
            0xFF10 => &mut self.nr10,
            0xFF11 => &mut self.nr11,
            0xFF12 => &mut self.nr12,
            0xFF13 => &mut self.nr13,
            0xFF14 => &mut self.nr14,
            0xFF16 => &mut self.nr21,
            0xFF17 => &mut self.nr22,
            0xFF18 => &mut self.nr23,
            0xFF19 => &mut self.nr24,
            0xFF1A => &mut self.nr30,
            0xFF1B => &mut self.nr31,
            0xFF1C => &mut self.nr32,
            0xFF1D => &mut self.nr33,
            0xFF1E => &mut self.nr34,
            0xFF20 => &mut self.nr41,
            0xFF21 => &mut self.nr42,
            0xFF22 => &mut self.nr43,
            0xFF23 => &mut self.nr44,
            0xFF24 => &mut self.nr50,
            0xFF25 => &mut self.nr51,
            0xFF26 => &mut self.nr52,
            0xFF30..=0xFF3F => &mut self.wave_ram[(addr - 0xFF30) as usize],
            0xFF40 => &mut self.lcdc,
            0xFF41 => &mut self.stat,
            0xFF42 => &mut self.scy,
            0xFF43 => &mut self.scx,
            0xFF44 => &mut self.ly,
            0xFF45 => &mut self.lyc,
            0xFF46 => &mut self.dma,
            0xFF47 => &mut self.bgp,
            0xFF48 => &mut self.obp0,
            0xFF49 => &mut self.obp1,
            0xFF4A => &mut self.wy,
            0xFF4B => &mut self.wx,
            0xFF4C => &mut self.key0,
            0xFF4D => &mut self.key1,
            0xFF4F => &mut self.vbk,
            0xFF50 => &mut self.bank,
            0xFF51 => &mut self.hdma1,
            0xFF52 => &mut self.hdma2,
            0xFF53 => &mut self.hdma3,
            0xFF54 => &mut self.hdma4,
            0xFF55 => &mut self.hdma5,
            0xFF56 => &mut self.rp,
            0xFF68 => &mut self.bcps,
            0xFF69 => &mut self.bcpd,
            0xFF6A => &mut self.ocps,
            0xFF6B => &mut self.ocpd,
            0xFF6C => &mut self.opri,
            0xFF70 => &mut self.svbk,
            0xFF76 => &mut self.pcm12,
            0xFF77 => &mut self.pcm34,
            0xFFFF => &mut self.ie,
            _ => return None,
        };
        Some(data)
    }

    fn read_hardware_register(&mut self, addr: u16) -> u8 {
        if let Some(reg) = self.get_hardware_register(addr) {
            *reg
        } else {
            0xFF
        }
    }

    fn write_hardware_register(&mut self, addr: u16, value: u8) {
        if let Some(reg) = self.get_hardware_register(addr) {
            *reg = value
        }
    }
}
