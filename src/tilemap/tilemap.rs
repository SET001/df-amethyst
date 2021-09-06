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
  fn sprite(&self, pos: Point3<u32>, _: &World) -> Option<usize> {
    if pos[2] == 1 {
      let x = pos[1];
      const LAST_LINE: u32 = TILEMAP_HEIGHT - 1;
      const PRE_LAST_LINE: u32 = LAST_LINE - 1;
      const FIRST_LINE: u32 = 0;
      match x {
        FIRST_LINE => return Some(2),
        1 => return Some(0),
        2 => return Some(1),
        PRE_LAST_LINE => return Some(4),
        LAST_LINE => return Some(3),
        _ => return Some(5),
      }
    }
    return Some(5);
  }
}
