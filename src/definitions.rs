#![allow(dead_code)]

const ROM_END: u16 = 0x7fff;
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9fff;
const RAM_START: u16 = 0xc000;
const RAM_END: u16 = 0xcfff;
const ZERO_PAGE_START: u16 = 0xff80;
const ZERO_PAGE_END: u16 = 0xfffe;


pub const SP_INIT: u16 = 0xfffe;
pub const PC_INIT: u16 = 0x0100;