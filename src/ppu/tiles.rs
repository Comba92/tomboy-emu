fn tile_to_vector(tile: &[u8; 16*16]) -> Vec<u8> {
  let lsbit = tile.iter().step_by(2);
  let msbit = tile.iter().skip(1).step_by(2);

  let tile: Vec<u8> = msbit
    .zip(lsbit)
    .map(|(high, low)| (high << 1 & low))
    .collect();

  tile
}