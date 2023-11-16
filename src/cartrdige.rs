#[allow(dead_code)]
#[derive(Debug)]
pub struct Cartridge {
  rom: Vec<u8>,
  title: String,
  publisher: u16,
  cgb_flag: u8,
  sgb_flag: u8,
  cart_type: u8,
  rom_size: u8,
  actual_rom_size: u32,
  ram_size: u8,
  continent: u8,
  old_publisher: u8,
  version: u8,
  checksum: u8,
  checksum_pass: bool,
}


fn extract_string(data: &Vec<u8>, start: usize, end: usize) -> String {
  String::from_utf8(data[start .. end].to_vec())
    .unwrap().replace("\0", "")
}

impl Cartridge {
  pub fn new(data: Vec<u8>) -> Self {
    if data.len() < 0x14f {
      panic!("Rom is too small, and doesn't contain a full header.");
    } 

    let title = extract_string(&data, 0x134, 0x143);
    let publisher = u16::from_be_bytes([data[0x144], data[0x145]]);
    let cgb_flag = data[0x143];
    let sgb_flag = data[0x146];
    let cart_type = data[0x147];
    let rom_size = data[0x148];
    let actual_rom_size = 32 * 1024 * (1 << rom_size);
    let ram_size = data[0x149];
    let continent = data[0x14a];
    let old_publisher = data[0x14b];
    let version = data[0x14c];
    let checksum = data[0x14d];

    let mut check = 0u8;
    for i in 0x134 ..= 0x14c {
      check = check.wrapping_sub(data[i]).wrapping_sub(1);
    }
    let checksum_pass = checksum == check;

    Cartridge {
      rom: data,
      title,
      publisher,
      cgb_flag,
      sgb_flag,
      cart_type,
      rom_size,
      actual_rom_size,
      ram_size,
      continent,
      old_publisher,
      version,
      checksum,
      checksum_pass
    }
  }
}

