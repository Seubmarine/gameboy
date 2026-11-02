use std::fmt::Write;


pub struct Ppu {}

#[derive(Debug)]
pub struct Tile {
    pixels : [u8; 64],
}

impl Tile {
    const BYTE_SIZE : usize = 16;

    const PIXEL_SIZE_X : usize = 8;
    const PIXEL_SIZE_Y : usize = 8;
    const PIXEL_COUNT : usize = Tile::PIXEL_SIZE_X * Tile::PIXEL_SIZE_Y;

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

    pub fn new(raw : &[u8; Tile::BYTE_SIZE] ) -> Tile {
        Tile { pixels: Tile::decode(raw) }
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
        };
        f.write_char('\n')
    }
}

pub struct Tilemap {
    data : [u8; 1024],
    tiles : [Tile; 32*32],
}

impl Tilemap {
    pub fn new(vram : &[u8], data: &[u8; 1024]) -> Tilemap {
        let tiles = data.map(|tile_index| Ppu::get_tile(vram, tile_index as usize));
        Tilemap {data: data.clone(), tiles}
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
                    }
                }
            s.push('\n');
            }
        }

        f.write_str(&s)
    }
}

impl Ppu {
    pub fn get_tile(vram : &[u8], index : usize) -> Tile {
        let index = index * Tile::BYTE_SIZE;
        let data = &vram[index..index + Tile::BYTE_SIZE];
        let raw_byte = data.try_into().expect("couldn't convert vram to a tile data");
        Tile::new(raw_byte)
    }

    const VRAM_INDEX : usize = 0x8000;

    const TILEMAP1_RANGE : std::ops::RangeInclusive<usize> = (0x9800 - Ppu::VRAM_INDEX)..=(0x9BFF - Ppu::VRAM_INDEX);
    const TILEMAP2_RANGE : std::ops::RangeInclusive<usize> = (0x9C00 - Ppu::VRAM_INDEX)..=(0x9FFF - Ppu::VRAM_INDEX);

    // const TILEMAP1_INDEX : usize = 0x9800 - Ppu::VRAM_INDEX; 
    // const TILEMAP2_INDEX : usize = 0x9C00 - Ppu::VRAM_INDEX;
    pub fn get_tilemaps(vram : &[u8], ) -> (Tilemap, Tilemap) {
        let t1_data = &vram[Ppu::TILEMAP1_RANGE];
        let t1_slice = t1_data.try_into().unwrap();
        let t1 = Tilemap::new(vram, t1_slice);
        
        let t2_data = &vram[Ppu::TILEMAP2_RANGE];
        let t2 = Tilemap::new(vram, t2_data.try_into().unwrap());

        
        (t1, t2)
    }
}