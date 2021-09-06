use amethyst::{
  core::math::{Point3, Vector3},
  prelude::*,
  tiles::Tile,
};

pub const TILEMAP_HEIGHT: u32 = 8;
pub const TILEMAP_WIDTH: u32 = 1000;

#[derive(Default, Clone)]
pub struct ExampleTile;
impl Tile for ExampleTile {
  // fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
  //   Some(1)
  // }

  fn sprite(&self, pos: Point3<u32>, _: &World) -> Option<usize> {
    if pos[2] == 1 {
      let x = pos[1];
      const LAST_LINE: u32 = TILEMAP_HEIGHT - 1;
      const PRE_LAST_LINE: u32 = TILEMAP_HEIGHT - 2;
      match x {
        0 => return Some(2),
        1 => return Some(0),
        2 => return Some(1),
        LAST_LINE => return Some(3),
        PRE_LAST_LINE => return Some(4),
        _ => return Some(5),
      }
    }
    return Some(5);
  }
}
