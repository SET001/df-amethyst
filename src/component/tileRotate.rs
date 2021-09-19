#[derive(Clone)]
pub enum TileGenerators {
  icy,
}

#[derive(Clone)]
pub struct TileRotate {
  pub rotaionCycle: u32,
  pub tileGenerator: TileGenerators,
}

impl Default for TileRotate {
  fn default() -> TileRotate {
    TileRotate {
      rotaionCycle: 0,
      tileGenerator: TileGenerators::icy,
    }
  }
}
