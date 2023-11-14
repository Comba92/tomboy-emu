#[cfg(test)]
mod tests {
    use tomboy_emu::cpu::CPU;

  #[test]
  fn it_works() {
    let mut cpu = CPU::new();
    cpu.c = 0x50;
    cpu.load_and_run(vec![0x41]);

    assert_eq!(cpu.b, cpu.c);
  }

  #[test]
  fn nop() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x00]);
  }
}