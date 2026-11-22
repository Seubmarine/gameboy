use std::{env::args, fs, io::Read, panic::AssertUnwindSafe};

use crate::{cartridge::Cartridge, gameboy::GameBoy};
mod cartridge;
mod gameboy;
mod joypad;
mod mbc;
mod ppu;
mod sm83;

fn main() {
    let path = args()
        .nth(1)
        .unwrap_or("rom/Legend of Zelda, The - Link's Awakening (France).gb".to_string());
    let mut file = fs::File::open(path).unwrap();

    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes).unwrap();

    let c = Cartridge::load(&file_bytes).unwrap();
    for w in &c.warnings {
        println!("Warning!: {w:?}");
    }

    println!("{c}");

    let mut gb = GameBoy::load(&c);
    let r = std::panic::catch_unwind(AssertUnwindSafe(|| {
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
        gb.run_n_cycles(10000000);
    }));
    if let Err(e) = r {
        eprintln!("Gameboy panicked with:\n{e:?}");
    }
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

use ppu::Frame;

#[test]
fn sdl2_test() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window = video
        .window(
            "Emu",
            (Frame::SIZE_X * 4) as u32,
            (Frame::SIZE_Y * 4) as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // Optional: crisp pixel scaling
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "nearest");
    canvas
        .set_logical_size(Frame::SIZE_X as u32, Frame::SIZE_Y as u32)
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB555,
            Frame::SIZE_X as u32,
            Frame::SIZE_Y as u32,
        )
        .map_err(|e| e.to_string())?;

    let mut frame = ppu::tests::frame_linear_color();
    let mut events = sdl.event_pump()?;

    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    'running: loop {
        for e in events.poll_iter() {
            match e {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // ... run your emulator and fill frame.pixels here ...
        render(&frame, &mut texture)?;

        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }
    Ok(())
}

fn render(frame: &Frame, texture: &mut sdl2::render::Texture) -> Result<(), String> {
    // View the [u16] as bytes (native endian, which is what SDL expects)
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(frame.pixels.as_ptr() as *const u8, frame.pixels.len() * 2)
    };
    texture
        .update(None, bytes, Frame::SIZE_X * 2) // pitch in bytes
        .map_err(|e| e.to_string())
}
