use amethyst::{
  core::math::{Point2, Point3, Vector3},
  prelude::*,
  tiles::Tile,
};
use rand::Rng;

pub const TILEMAP_HEIGHT: u32 = 8;
pub const TILEMAP_WIDTH: u32 = 1000;

#[derive(Default, Clone)]
pub struct ExampleTile;

#[derive(Default, Clone)]
pub struct LevelTileMap {
  pub items: Vec<u32>,
}

pub fn get_map_index(x: u32, y: u32, width: u32) -> u32 {
  return x + y * width;
}

impl Tile for ExampleTile {
  fn sprite(&self, pos: Point3<u32>, world: &World) -> Option<usize> {
    let level = (*world.read_resource::<LevelTileMap>()).clone();
    // let mut rng = rand::thread_rng();
    // let snowTile = rng.gen_range(5, 10);
    // return Some(2);
    let index = get_map_index(pos[0], pos[1], TILEMAP_WIDTH) as usize;
    return Some(level.items[index] as usize);
    // if pos[2] == 1 {
    // let x = pos[1];
    // const LAST_LINE: u32 = TILEMAP_HEIGHT - 1;
    // const PRE_LAST_LINE: u32 = LAST_LINE - 1;
    // const FIRST_LINE: u32 = 0;
    // match x {
    //   FIRST_LINE => return Some(2),
    //   1 => return Some(0),
    //   2 => return Some(1),
    //   PRE_LAST_LINE => return Some(4),
    //   LAST_LINE => return Some(3),
    //   _ => return Some(5),
    // }
    // }
    // return Some(3);
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
