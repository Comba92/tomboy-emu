#![allow(dead_code)]

/* 
  0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
  4000	7FFF	16 KiB ROM Bank 01~NN	From cartridge, switchable bank via mapper (if any)
  8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
  A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
  C000	CFFF	4 KiB Work RAM (WRAM)	
  D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1~7
  E000	FDFF	Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
  FE00	FE9F	Object attribute memory (OAM)	
  FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited
  FF00	FF7F	I/O Registers	
  FF80	FFFE	High RAM (HRAM)	
  FFFF	FFFF	Interrupt Enable register (IE)	 
*/

pub const ROM_START:          u16 = 0x0000;
pub const ROM_END:            u16 = 0x7fff;
pub const VRAM_START:         u16 = 0x8000;
pub const VRAM_END:           u16 = 0x9fff;
pub const EXT_RAM_START:      u16 = 0xa000;
pub const EXT_RAM_END:        u16 = 0xbfff;
pub const WRAM_START:         u16 = 0xc000;
pub const WRAM_END:           u16 = 0xdfff;
pub const ECHO_RAM_START:     u16 = 0xe000;
pub const ECHO_RAM_END:       u16 = 0xfdff;
pub const OAM_START:          u16 = 0xfe00;
pub const OAM_END:            u16 = 0xfe9f;
pub const ILLEGAL_AREA_START: u16 = 0xfea0;
pub const ILLEGAL_AREA_END:   u16 = 0xfeff;
pub const IO_REGISTERS_START: u16 = 0xff00;
pub const IO_REGISTERS_END:   u16 = 0xff7f;
pub const HRAM_START:         u16 = 0xff80;
pub const HRAM_END:           u16 = 0xfffe;

pub const INTERRUPT_ENABLE: u16 = 0xffff;
pub const INTERRUPT_FLAG:   u16 = 0xff0f;

pub const A_INIT: u8 = 0x01;
pub const B_INIT: u8 = 0x00;
pub const C_INIT: u8 = 0x13;
pub const D_INIT: u8 = 0x00;
pub const E_INIT: u8 = 0xd8;
pub const F_INIT: u8 = 0xb0;
pub const H_INIT: u8 = 0x01;
pub const L_INIT: u8 = 0x4d;
pub const SP_INIT: u16 = 0xfffe;
pub const PC_INIT: u16 = 0x0100;
pub const DIV_INIT: u16 = 0xabcc;

pub const LCD_WIDTH:  usize = 160;
pub const LCD_HEIGHT: usize = 144;