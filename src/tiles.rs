use amethyst::{
  core::math::Point3,
  prelude::*,
  // renderer::palette::Srgba,
  tiles::Tile,
};
use rand::{thread_rng, Rng};

const ICY_DEFAULT: usize = 0;
const ICY_BOTTOM: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const ICY_TOP_TOP: usize = 8;
const ICY_TOP: usize = 9;
const ICY_TOP_BOTTOM: usize = 10;
const ICY_WALL_BOTTOM: usize = 11;
const ICY_WALL: usize = 12;

const STEPPE_WATTER: usize = 13;
const STEPPE_BOTTOM: usize = 14;
const STEPPE_STONE1: usize = 15;
const STEPPE_SMALL_STONES: usize = 16;
const STEPPE_STONE2: usize = 17;
const STEPPE_STONE3: usize = 18;
const STEPPE_ARTIFACT_1_1: usize = 19;
const STEPPE_ARTIFACT_1_2: usize = 20;

const STEPPE_ARTIFACT_2_1: usize = 21;
const STEPPE_ARTIFACT_2_2: usize = 22;

#[derive(Default, Clone)]
pub struct IcyTile;

// pub fn get_map_index(x: u32, y: u32, width: u32) -> u32 {
//   return x + y * width;
// }

impl Tile for IcyTile {
  fn sprite(&self, pos: Point3<u32>, _: &World, _: &Resources) -> Option<usize> {
    // let level = (*world.read_resource::<MapLayers>()).0;
    // let index = get_map_index(pos[0], pos[1], TILEMAP_WIDTH) as usize;
    // return Some(level[pos[2] as usize][index] as usize);
    if pos[2] == 0 {
      Some(ICY_DEFAULT)
    } else {
      Some(match pos.y {
        0 => ICY_TOP_BOTTOM,
        1 => ICY_WALL,
        2 => ICY_WALL_BOTTOM,
        6 => ICY_TOP_TOP,
        7 => ICY_TOP,
        _ => ICY_DEFAULT,
      })
    }
  }
}

#[derive(Default, Clone)]
pub struct SteppeTile;
impl Tile for SteppeTile {
  fn sprite(&self, pos: Point3<u32>, _: &World, _: &Resources) -> Option<usize> {
    Some(STEPPE_BOTTOM)
  }
}

#[derive(Default, Clone)]
pub struct MixedSteppeIcyTile;
impl Tile for MixedSteppeIcyTile {
  fn sprite(&self, pos: Point3<u32>, _: &World, _: &Resources) -> Option<usize> {
    Some(ICY_DEFAULT)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  // #[test]
  // fn test_get_map_index() {
  //   assert_eq!(get_map_index(0, 0, 10), 0);
  //   assert_eq!(get_map_index(1, 0, 10), 1);
  //   assert_eq!(get_map_index(1, 1, 10), 11);
  // }
}
