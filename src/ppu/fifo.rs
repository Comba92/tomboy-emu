enum FetcherState {
  ReadTile, ReadData0, ReadData1, Sleep, Push
}

struct Fetcher {
  state: FetcherState,
  cycles: usize,
}

use FetcherState::*;

impl Fetcher {
  pub fn new() -> Fetcher {
    Fetcher { state: ReadTile, cycles: 0 }
  }

  pub fn step(&mut self, cycles: usize) {
    match self.state {
      ReadTile => {},
      ReadData0 => {},
      ReadData1 => {},
      Sleep => {},
      Push => {}
    }
  }
}