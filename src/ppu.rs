use std::fmt::Write;
use crate::gameboy::Bus;


enum PpuMode {
    OAM_SCAN = 2,
    DRAW_PIXELS = 3,
    HORIZONTAL_BLANK = 0,
    VERTICAL_BLANK = 1,
}

pub struct Ppu {
    lx: u8,   //line x
    ly: u8,   //line y
    lyc: u8,  //line y compare
    scx: u8,  //Background viewport x (scrolling)
    scy: u8,  //Background viewport y (scrolling)
    lcdc: u8, //LCD Control
    //GameBoy (non-color) palette
    bgp: u8,  //backgroud palette
    obp0: u8, //object palette 0
    obp1: u8, //object palette 1
    //GameBoy color palette
    bgpi: u8, //backgroud palette index
    bgpd: u8, //backgroud palette data
    obpi: u8, //object palette index
    obpd: u8, //object palette data
    //Window
    wy: u8, //window y
    wx: u8, //window x

    //ppu internal
    mode: PpuMode,
    frame: Frame,
    pub oam: [u8; Ppu::OAM_SIZE],
    pub dma : Dma,
}

impl Ppu {
    const OAM_SIZE: usize = 160;
    pub fn new() -> Ppu {
        Ppu {
            lx: 0,   //line x
            ly: 0,   //line y
            lyc: 0,  //line y compare
            scx: 0,  //Background viewport x (scrolling)
            scy: 0,  //Background viewport y (scrolling)
            lcdc: 0, //LCD Control
            //GameBoy (non-color) palette
            bgp: 0,  //backgroud palette
            obp0: 0, //object palette 0
            obp1: 0, //object palette 1
            //GameBoy color palette
            bgpi: 0, //backgroud palette index
            bgpd: 0, //backgroud palette data
            obpi: 0, //object palette index
            obpd: 0, //object palette data
            //Window
            wy: 0, //window y
            wx: 0, //window x
            mode: PpuMode::OAM_SCAN,
            frame: Frame::new(),
            oam: [0; Ppu::OAM_SIZE],
            dma: Dma::new(),
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        0x9F;
        match addr {
            0xFF40 => self.lcdc,
            0xFF41 => todo!("stat read"),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma.read_dma(),
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            // ...
            _ => 0xFF,
        }
    }

    //register 0xFF40..=0xFF70
    pub fn write_reg(&mut self, addr: u16, v: u8) {
        match addr {
            0xFF40 => self.lcdc = v,
            0xFF41 => todo!("stat read"),
            0xFF42 => self.scy = v,
            0xFF43 => self.scx = v,
            0xFF44 => {}
            0xFF45 => self.lyc = v,
            0xFF46 => self.dma.start_dma(v),
            0xFF47 => self.bgp = v,
            0xFF48 => self.obp0 = v,
            0xFF49 => self.obp1 = v,
            0xFF4A => self.wy = v,
            0xFF4B => self.wx = v,
            0xFF6C => todo!("object priority mode"),
            0xFF70 => todo!("wram bank"),
            _ => unreachable!(),
        }
    }

    const SCAN_LINE_H: u8 = 154;
    const SCAN_LINE_V: u8 = 153;
    const VERTICAL_BLANK_BEGIN: u8 = 144;

    pub fn cycle_m(&mut self) {
        self.cycle_t();
        self.cycle_t();
        self.cycle_t();
        self.cycle_t();
    }

    pub fn cycle_t(&mut self) {
        const OAM_SCAN_DURATION: u8 = 80;
        if self.lx < OAM_SCAN_DURATION {
        } else {
            //either mode 3(draw) or 0(h blank)
        }
        // let mode = if self.y_position >= Ppu::VERTICAL_BLANK_BEGIN {
        //     PpuMode::VERTICAL_BLANK
        // } else {
        //     match self.x_position {
        //         0..80 => PpuMode::OAM_SCAN,
        //         172..289 => PpuMode::DRAW_PIXELS,
        //         0..80 => PpuMode::,
        //     };
        // }
        self.lx += 1;

        if self.lx >= Ppu::SCAN_LINE_H {
            self.lx = 0;
            self.ly += 1;

            if self.ly >= Ppu::SCAN_LINE_V {
                self.ly = 0;
            }
        }
    }

    pub fn get_tile(vram: &[u8], index: usize) -> Tile {
        let index = index * Tile::BYTE_SIZE;
        let data = &vram[index..index + Tile::BYTE_SIZE];
        let raw_byte = data
            .try_into()
            .expect("couldn't convert vram to a tile data");
        Tile::new(raw_byte)
    }

    const VRAM_INDEX: usize = 0x8000;

    const TILEMAP1_RANGE: std::ops::RangeInclusive<usize> =
        (0x9800 - Ppu::VRAM_INDEX)..=(0x9BFF - Ppu::VRAM_INDEX);
    const TILEMAP2_RANGE: std::ops::RangeInclusive<usize> =
        (0x9C00 - Ppu::VRAM_INDEX)..=(0x9FFF - Ppu::VRAM_INDEX);

    // const TILEMAP1_INDEX : usize = 0x9800 - Ppu::VRAM_INDEX;
    // const TILEMAP2_INDEX : usize = 0x9C00 - Ppu::VRAM_INDEX;
    pub fn get_tilemaps(vram: &[u8]) -> (Tilemap, Tilemap) {
        let t1_data = &vram[Ppu::TILEMAP1_RANGE];
        let t1_slice = t1_data.try_into().unwrap();
        let t1 = Tilemap::new(vram, t1_slice);

        let t2_data = &vram[Ppu::TILEMAP2_RANGE];
        let t2 = Tilemap::new(vram, t2_data.try_into().unwrap());

        (t1, t2)
    }

    pub fn get_object(&self, index: usize) -> Object {
        assert!(index < 40, "Only 40 object in the Object Attribute Memory");
        let index = index * 4; //an Object is 4 bytes
        Object {
            y_position : self.oam[index],
            x_position : self.oam[index + 1],
            tile_index : self.oam[index + 2],
            attributes : self.oam[index + 3],
        }
    }
}


pub struct Dma {
    start_delay: bool,
    is_working: bool,
    start_address: u16,
    new_start_address: u16,
    index: u8,
}

impl Dma {
    fn new() -> Dma {
        Dma {
            start_delay: false,
            is_working: false,
            start_address: 0xFFFF,
            index: 0,
            new_start_address: 0xFFFF,
        }
    }

    //If the dma is currently in work, this return Some(address to copy, oam index)
    //if it's not in work it returns None
    //a complete dma copy should take 160 m_cycles for the 160 bytes in OAM 
    pub fn cycle_m(&mut self) -> Option<(u16, usize)> {
        //copy bytes each cycle
        if self.is_working {
            if (self.index as usize) < Ppu::OAM_SIZE {
                let info =  Some((self.start_address + self.index as u16, self.index as usize));
                self.index += 1;
                return info
            } else {
                self.is_working = false;
                return None
            }
        }

        //condition is after dma is_working only start the work in the next cycle
        if self.start_delay {
            self.start_delay = false;
            self.is_working = true;
            self.start_address = self.new_start_address;
            self.index = 0;
        }
        None
    }

    //This start the dma in the next cycle when writing to bus 0xFF46
    //writing the value 0x42 will start the dma at the adress 0x4200
    fn start_dma(&mut self, value: u8) {
        self.start_delay = true;
        self.new_start_address = (value as u16) << 8;
    }

    //get the last value written start_dma, it's called when reading bus 0xFF46
    fn read_dma(&self) -> u8 {
        (self.new_start_address >> 8) as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Object {
    pub y_position: u8,
    pub x_position: u8,
    pub tile_index: u8,
    pub attributes: u8,
} 

impl Object {
    //Flags
    pub const PRIOTITY : u8 = 1 << 7;
    pub const FLIP_Y : u8 = 1 << 6;
    pub const FLIP_X : u8 = 1 << 5;
    pub const DMG_PALETTE : u8 = 1 << 4;
    pub const BANK : u8 = 1 << 3;
    
    pub fn get_cgb_pallette(&self) -> u8 {
        const CGB_PALETTE_MASK : u8 = 0b111;
        self.attributes & CGB_PALETTE_MASK
    }
}

pub struct Frame {
    pixels: [u16; Frame::SIZE_X * Frame::SIZE_Y],
}

impl Frame {
    const SIZE_X: usize = 160;
    const SIZE_Y: usize = 144;
    const SIZE: usize = Frame::SIZE_X * Frame::SIZE_Y;

    pub fn new() -> Frame {
        Frame {
            pixels: [0; Frame::SIZE],
        }
    }

    pub fn set_pixels(&mut self, x: usize, y: usize, value: u16) {
        assert!(
            x >= Frame::SIZE_X,
            "setting pixel with x out of bound(frame x limit: {}).",
            Frame::SIZE_X
        );
        assert!(
            y >= Frame::SIZE_Y,
            "setting pixel with y out of bound(frame y limit: {}).",
            Frame::SIZE_Y
        );
        self.pixels[x * Frame::SIZE_Y + y] = value;
    }

    pub fn get_raw(&self) -> [u16; Frame::SIZE] {
        self.pixels
    }
}

#[derive(Debug)]
pub struct Tile {
    pixels: [u8; 64],
}

impl Tile {
    const BYTE_SIZE: usize = 16;

    const PIXEL_SIZE_X: usize = 8;
    const PIXEL_SIZE_Y: usize = 8;
    const PIXEL_COUNT: usize = Tile::PIXEL_SIZE_X * Tile::PIXEL_SIZE_Y;

    fn decode(raw: &[u8; Tile::BYTE_SIZE]) -> [u8; 64] {
        let mut pixels = [0u8; 64];
        for i in 0..Tile::BYTE_SIZE / 2 {
            let mut byte_0 = raw[i * 2];
            let mut byte_1 = raw[i * 2 + 1];
            byte_0 = byte_0.reverse_bits();
            byte_1 = byte_1.reverse_bits();
            for shift in 0..8 {
                let palette_index = (byte_0 & 0b1) + (byte_1 & 0b1);
                pixels[i * 8 + shift] = palette_index;
                byte_0 >>= 1;
                byte_1 >>= 1;
            }
        }
        pixels
    }

    pub fn new(raw: &[u8; Tile::BYTE_SIZE]) -> Tile {
        Tile {
            pixels: Tile::decode(raw),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Tile::PIXEL_SIZE_Y {
            for x in 0..Tile::PIXEL_SIZE_X {
                let b = self.pixels[y * Tile::PIXEL_SIZE_X + x];
                //.:-=+*#%@
                match b & 0b11 {
                    0 => f.write_char(' ')?,
                    1 => f.write_char('.')?,
                    2 => f.write_char('=')?,
                    3 => f.write_char('#')?,
                    _ => unreachable!(),
                };
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

pub struct Tilemap {
    data: [u8; 1024],
    tiles: [Tile; 32 * 32],
}

impl Tilemap {
    pub fn new(vram: &[u8], data: &[u8; 1024]) -> Tilemap {
        let tiles = data.map(|tile_index| Ppu::get_tile(vram, tile_index as usize));
        Tilemap {
            data: data.clone(),
            tiles,
        }
    }
}

impl std::fmt::Display for Tilemap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for map_y in 0..32 {
            for tile_y in 0..8 {
                for map_x in 0..32 {
                    let current = &self.tiles[map_y * 32 + map_x];
                    for tile_x in 0..8 {
                        let pixel = current.pixels[tile_y * 8 + tile_x];
                        let c = match pixel & 0b11 {
                            0 => '.',
                            1 => '-',
                            2 => '=',
                            3 => '#',
                            _ => unreachable!(),
                        };
                        s.push(c);
                        s.push(' ');
                    }
                }
                s.push('\n');
            }
        }

        f.write_str(&s)
    }
}
