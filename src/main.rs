use std::env;
use std::fs;
use sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use tomboy_emu::bus::BUS;
use tomboy_emu::cpu::CPU;


const palette: [Color; 4] = [Color::RED, Color::GRAY, Color::GREY, Color::WHITE];

fn draw_pixel(tile: &Vec<u8>) -> Vec<u8> {
  let lsbit = tile.iter().step_by(2);
  let msbit = tile.iter().skip(1).step_by(2);

  let tile: Vec<u8> = msbit
    .zip(lsbit)
    .flat_map(|(high, low)| {
      let mut row = vec![];
      for i in 0..8 {
        let hb = (high >> (7-i)) & 1; 
        let lb = (low >> (7-i)) & 1;
        row.push(hb << 1 | lb);
      }
      row
    })
    .collect();

  tile
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() <= 1 {
    eprintln!("No rom file provided.");
    std::process::exit(1);
  }


  let rom_path = &args[1];
  let rom = fs::read(rom_path)
    .expect("Error reading the file.");

  let memory = BUS::new(rom);
  let mut cpu = CPU::new(memory);

  let now = std::time::SystemTime::now();
  if args.len() > 2 {
    let delay = args.get(2).unwrap().parse().unwrap();
    eprintln!("Running...");
    loop {
      if now.elapsed().unwrap() > std::time::Duration::from_secs(delay) {
        std::process::exit(0);
      } 
      cpu.step();
    }
  }

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("Tomboy - GB Emulator", 32 * 10, 32 * 10)
    .position_centered()
    .build().unwrap();

  let mut canvas = window.into_canvas()
    .accelerated()
    .target_texture()
    .build().unwrap();
  canvas.set_scale(10., 10.).unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();

  loop {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} => std::process::exit(0),
        _ => ()
      }
    }

    cpu.step();
    
    //canvas.present();
    //std::thread::sleep(Duration::from_millis(1));
  }
}