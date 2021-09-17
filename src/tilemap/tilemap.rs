// use crate::level::MapLayers;
use amethyst::{
  core::math::{Point2, Point3, Vector3},
  prelude::*,
  renderer::palette::Srgba,
  tiles::Tile,
};
use rand::Rng;

pub const TILEMAP_HEIGHT: u32 = 8;
pub const TILEMAP_WIDTH: u32 = 10;

const ICE_WALL_TILE: usize = 0;
const ICE_WALL_AGE_TILE: usize = 1;
const UPPER_COAST_TILE: usize = 2;
const BOTTOM_COAST_TILE: usize = 3;
const BOTTOM_COAST_AGE_TILE: usize = 4;
const DEFAULT_TILE: usize = 5;
const BOTTOM_SNOW_TILES: [usize; 9] = [5, 6, 7, 8, 9, 10, 11, 12, 13];

#[derive(Default, Clone)]
pub struct ExampleTile;

pub fn get_map_index(x: u32, y: u32, width: u32) -> u32 {
  return x + y * width;
}

impl Tile for ExampleTile {
  fn sprite(&self, pos: Point3<u32>, world: &World, resources: &Resources) -> Option<usize> {
    // let level = (*world.read_resource::<MapLayers>()).0;
    // let index = get_map_index(pos[0], pos[1], TILEMAP_WIDTH) as usize;
    // return Some(level[pos[2] as usize][index] as usize);
    if pos[2] == 0 {
      Some(DEFAULT_TILE)
    } else {
      Some(match pos.y {
        0 => UPPER_COAST_TILE,
        1 => ICE_WALL_TILE,
        2 => ICE_WALL_AGE_TILE,
        6 => BOTTOM_COAST_AGE_TILE,
        7 => BOTTOM_COAST_TILE,
        _ => DEFAULT_TILE,
      })
    }
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
