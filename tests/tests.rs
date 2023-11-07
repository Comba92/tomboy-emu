#[cfg(test)]
mod tests {
    use tomboy_emu::cpu::CPU;

  #[test]
  fn it_works() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x10;
    cpu.load(vec!(0xea, 0x00, 0x20));
    cpu.run();

    println!("{:#x?}", &cpu.ram[0 .. 0x25]);
    assert_eq!(cpu.ram[0x20], 0x10);
  }
}