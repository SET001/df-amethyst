// use crate::level::MapLayers;
use amethyst::{
  core::math::{Point2, Point3, Vector3},
  prelude::*,
  tiles::Tile,
};
use rand::Rng;

pub const TILEMAP_HEIGHT: u32 = 8;
pub const TILEMAP_WIDTH: u32 = 50;

#[derive(Default, Clone)]
pub struct ExampleTile;

pub fn get_map_index(x: u32, y: u32, width: u32) -> u32 {
  return x + y * width;
}

impl Tile for ExampleTile {
  fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
    // let level = (*world.read_resource::<MapLayers>()).0;
    // let index = get_map_index(pos[0], pos[1], TILEMAP_WIDTH) as usize;
    // return Some(level[pos[2] as usize][index] as usize);
    Some(0)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_get_map_index() {
    assert_eq!(get_map_index(0, 0, 10), 0);
    assert_eq!(get_map_index(1, 0, 10), 1);
    assert_eq!(get_map_index(1, 1, 10), 11);
  }
}
