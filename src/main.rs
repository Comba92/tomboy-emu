use std::env;
use std::fs;
use sdl2;

use tomboy_emu::cartrdige::Cartridge;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() <= 1 {
    eprintln!("No rom file provided.");
    std::process::exit(1);
  }

  let rom_path = &args[1];
  let rom = fs::read(rom_path)
    .expect("Error reading the file.");

  let cartridge_data = Cartridge::new(rom);
  dbg!(cartridge_data);

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("Tomboy - GB Emulator", 32 * 10, 32 * 10)
    .position_centered()
    .build().unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  canvas.clear();
  canvas.present();
}