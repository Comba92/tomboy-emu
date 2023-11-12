#[cfg(test)]
mod tests {
    use tomboy_emu::cpu::CPU;

  #[test]
  fn it_works() {
    let mut cpu = CPU::new();
    cpu.registers.set(1, 0x20);

    cpu.load_and_run(vec!(0x03));
    assert_eq!(cpu.registers.get_16(0), 0x21);
  }

  #[test]
  fn nop() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec!(0x00));
  }
}