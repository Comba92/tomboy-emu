use std::env;
use std::fs;
use sdl2;
use sdl2::pixels::Color;

use tomboy_emu::Emulator;
use tomboy_emu::definitions::CYCLES_PER_FRAME;
use tomboy_emu::definitions::LCD_HEIGHT;
use tomboy_emu::definitions::LCD_WIDTH;


const PALETTE: [Color; 4] = [Color::WHITE, Color::GRAY, Color::GREY, Color::BLACK];

fn tile_to_2bpp(tile: &[u8]) -> Vec<Vec<u8>> {
  let lsbit = tile.iter().step_by(2);
  let msbit = tile.iter().skip(1).step_by(2);

  let tile = msbit
    .zip(lsbit)
    .map(|(high, low)| {
      let mut row = vec![];
      for i in 0..8 {
        let hb = (high >> (7-i)) & 1; 
        let lb = (low >> (7-i)) & 1;
        row.push(hb << 1 | lb);
      }
      row
    })
    .collect::<Vec<_>>();

  tile
}

fn dump_vram_tiles(emu: &Emulator, ctx: &mut SDL2Context) {
  let mut curr_x = 0;
  let mut curr_y = 0;
  
  let _ = &emu.memory.borrow()
    .vram[..]
    .chunks(16)
    .map(tile_to_2bpp)
    .enumerate()
    .for_each(|(_, tile)| {
      if curr_x >= 8 * 32 { curr_x = 0; curr_y += 8; }
      draw_tile(tile, curr_x, curr_y, ctx);
      curr_x += 8;
    });
}

fn draw_tile(tile: Vec<Vec<u8>>, x: i32, y: i32, ctx: &mut SDL2Context) {
  tile.iter().enumerate().for_each(|(off_y, row)| {
    row.iter().enumerate().for_each(|(off_x, &pixel)| {
      ctx.canvas.set_draw_color(PALETTE[pixel as usize]);
      ctx.canvas.fill_rect(sdl2::rect::Rect::new(
        x + off_x as i32,
        y + off_y as i32,
        1, 1
      )).unwrap(); 
    })
  })
}

struct SDL2Context {
  pub canvas: sdl2::render::WindowCanvas,
  pub event_pump: sdl2::EventPump
}
impl SDL2Context {
  pub fn new() -> Self {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tomboy - GB Emulator", LCD_WIDTH as u32 * 5, LCD_HEIGHT as u32 * 5)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .target_texture()
        .build().unwrap();
    canvas.set_scale(5., 5.).unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    SDL2Context {
      canvas,
      event_pump
    }
  }
}

fn main() {
  env_logger::builder().filter_level(log::LevelFilter::Off).init();

  let args: Vec<String> = env::args().collect();

  if args.len() <= 1 {
    eprintln!("No rom file provided.");
    std::process::exit(1);
  }


  let rom_path = &args[1];
  let rom = fs::read(rom_path)
    .expect("Error reading the file.");

  let mut emu = Emulator::new(rom);

  // for debugging without screen
  if args.len() > 2 {
    emu.run();
    std::process::exit(0);
  }

  let mut ctx = SDL2Context::new();

  loop {
    ctx.canvas.set_draw_color(Color::BLACK);
    ctx.canvas.clear();

    for event in ctx.event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} => std::process::exit(0),
        _ => ()
      }
    }

    for _ in 0..CYCLES_PER_FRAME  {
      if let Err(_) = emu.step() {
        continue;
      }
    }

    dump_vram_tiles(&emu, &mut ctx);

    ctx.canvas.present();
  }
}