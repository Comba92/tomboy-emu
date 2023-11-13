#[cfg(test)]
mod tests {
    use tomboy_emu::cpu::CPU;

  #[test]
  fn it_works() {
  }

  #[test]
  fn nop() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec!(0x00));
  }
}