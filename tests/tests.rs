#[cfg(test)]
mod tests {
    use tomboy_emu::cpu::CPU;

  #[test]
  fn it_works() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x10;
    cpu.load(vec!(0x02));
    cpu.run();

    assert_eq!(cpu.reg_bc(), 0x10);
  }
}