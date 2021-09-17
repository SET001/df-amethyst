pub trait Level {
  fn gen_map(&mut self);

  fn gen_tile(&self, index: u32) -> u32;
}

pub const MAP_LAYERS: usize = 2;
pub const MAP_WIDTH: usize = 50 * 8;

pub type LevelMap = [u32; MAP_WIDTH];

pub struct MapLayers(pub [LevelMap; MAP_LAYERS]);

impl MapLayers {
  pub fn new(default_tile: u32) -> MapLayers {
    MapLayers([[default_tile; MAP_WIDTH]; MAP_LAYERS])
  }
}

impl Default for MapLayers {
  fn default() -> MapLayers {
    MapLayers([[0; MAP_WIDTH]; MAP_LAYERS])
  }
}
