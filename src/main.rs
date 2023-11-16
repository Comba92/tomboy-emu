use std::env;
use std::fs;
use std::time::Duration;
use sdl2;

use tomboy_emu::mmu::MMU;
use tomboy_emu::cpu::CPU;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() <= 1 {
    eprintln!("No rom file provided.");
    std::process::exit(1);
  }

  let rom_path = &args[1];
  let rom = fs::read(rom_path)
    .expect("Error reading the file.");

  let memory = MMU::new(rom);
  let mut cpu = CPU::new(memory);

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("Tomboy - GB Emulator", 32 * 10, 32 * 10)
    .position_centered()
    .build().unwrap();

  let mut canvas = window.into_canvas()
    .accelerated()
    .build().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();
  canvas.set_scale(10., 10.).unwrap();

  loop {
    canvas.clear();

    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} => std::process::exit(0),
        _ => ()
      }
    }

    cpu.step();
    
    canvas.present();
    std::thread::sleep(Duration::from_millis(100));
  }
}