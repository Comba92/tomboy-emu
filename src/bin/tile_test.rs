fn main() {
  let tile_data = vec![
   0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C
  ];

  println!("{:?}", draw_pixel(&tile_data));
}


fn draw_pixel(tile: &Vec<u8>) -> Vec<Vec<u8>> {
  let lsbit = tile.iter().step_by(2);
  let msbit = tile.iter().skip(1).step_by(2);

  let tile = msbit
    .zip(lsbit)
    .map(|(high, low)| {
      let mut row = vec![];
      for i in 0..8 {
        let hb = (high >> (7-i)) & 1; 
        let lb = (low >> (7-i)) & 1;
        row.push(hb << 1 | lb);
      }
      row
    })
    .collect::<Vec<_>>();

  tile
}