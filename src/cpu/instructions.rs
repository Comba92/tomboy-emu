use super::CPU;

impl CPU {
  pub(super) fn load_r_to_r(&mut self, dst: u8, src: u8) {
    let data = self.registers.get(src);
    self.registers.set(dst, data);
  }

  pub(super) fn load_hl_indirect_to_r(&mut self, dst: u8) {
    let addr = self.registers.get_hl();
    let data = self.mem_read(addr);
    self.registers.set(dst, data);
  }

  pub(super) fn load_r_to_hl_indirect(&mut self, src: u8) {
    let addr = self.registers.get_hl();
    let data = self.registers.get(src);
    self.mem_write(addr, data)
  }

  pub(super) fn load_sp_to_mem(&mut self) {
    let addr = self.mem_read_16(self.pc);
    self.mem_write_16(addr, self.sp);
  }

  pub(super) fn inc_r(&mut self, reg: u8) {
    let value = self.registers.get(reg);
    self.registers.set(reg, value.wrapping_add(1));

    self.registers.set_zero(value == 0);
    self.registers.set_negative(false);
    self.registers.update_hcarry(value, 1);
  }

  pub(super) fn dec_r(&mut self, reg: u8) {
    let value = self.registers.get(reg);
    self.registers.set(reg, value.wrapping_sub(1));

    self.registers.set_zero(value == 0);
    self.registers.set_negative(true);
    self.registers.update_hcarry(value, !1 + 1);
  }

  pub(super) fn load_immediate_to_r(&mut self, reg: u8) {
    let data = self.mem_read(self.pc);
    self.registers.set(reg, data);
  }

  pub(super) fn load_immediate_16_to_rr(&mut self, reg: u8) {
    let data = self.mem_read_16(self.pc);
    self.registers.set_16(reg, data);
  }

  pub(super) fn load_immediate_16_to_sp(&mut self) {
    let data = self.mem_read_16(self.pc);
    self.sp = data;
  }

  pub(super) fn inc_rr(&mut self, reg: u8) {
    let value = self.registers.get_16(reg);
    self.registers.set_16(reg, value.wrapping_add(1));
  }

  pub(super) fn dec_rr(&mut self, reg: u8) {
    let value = self.registers.get_16(reg);
    self.registers.set_16(reg, value.wrapping_sub(1));
  }

}