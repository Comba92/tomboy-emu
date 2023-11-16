#![allow(dead_code)]

pub const ROM_START:          u16 = 0x0000;
pub const ROM_END:            u16 = 0x3fff;
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
pub const INTERRUPT_FLAG: u16   = 0xff0f;

pub const SP_INIT: u16 = 0xfffe;
pub const PC_INIT: u16 = 0x0100;

pub const LCD_WIDTH:  usize = 160;
pub const LCD_HEIGHT: usize = 144;