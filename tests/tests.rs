#[cfg(test)]
mod tests {
  use tomboy_emu::{cpu::CPU, mmu::MMU};
  
  fn init_cpu() -> CPU {
    let mem = MMU::new(vec![]);
    CPU::new(mem)
  }

  #[test]
  fn load_immediate_to_a() {
    let mut cpu = init_cpu();
    cpu.b = 0xff;
    cpu.load_and_run(vec![0x78, 0x10]);

    assert_eq!(cpu.a, 0xff);
  }

  #[test]
  fn jump_relative() {
    let mut cpu = init_cpu();
    cpu.load_and_run(vec![0x18, 0x04, 0x00, 0x00, 0x00, 0x10]);
    cpu.load_and_run(vec![0x18, 0x04, 0x00, 0x00, 0x10, 0x18, 0xFC])
  }

  #[test]
  fn jump_relative_zero() {
    let mut cpu = init_cpu();
    //cpu.f.insert(Flags::ZERO);
    cpu.load_and_run(vec![0x28, 0x04, 0x00, 0x00, 0x00, 0x10]);
    cpu.load_and_run(vec![0x28, 0x04, 0x00, 0x00, 0x10, 0x28, 0xFC]);
    cpu.load_and_run(vec![0x20, 0x01, 0x00, 0x10]);
  }

  #[test]
  fn nop() {
    let mut cpu = init_cpu();
    cpu.run();
  }
}