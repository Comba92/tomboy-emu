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
    cpu.load_and_run(vec![0x78]);

    assert_eq!(cpu.a, 0xff);
  }

  #[test]
  fn nop() {
    let mut cpu = init_cpu();
    cpu.run();
  }
}