// use crate::component::TileGenerator;
use crate::level::{LevelMap, MapLayers};
// use crate::tilemap::{ExampleTile, TILEMAP_HEIGHT, TILEMAP_WIDTH};
use amethyst::{
  core::Transform,
  ecs::{Join, System, Write, WriteExpect, WriteStorage},
  tiles::{MortonEncoder, TileMap},
};
use std::fmt::{self, Display, Formatter};
pub struct TileRotateSystem;
use rand::Rng;

// fn shift_layers(mut map: LevelMap) {}

// fn should_rotate() -> bool {
//   false
// }

// fn gen_map_col() {}

impl<'a> System<'a> for TileRotateSystem {
  type SystemData = (
    // ReadStorage<'a, TileMap<ExampleTile, MortonEncoder>>,
    // ReadStorage<'a, TileGenerator>,
    WriteStorage<'a, Transform>,
    Write<'a, MapLayers>,
  );

  fn run(&mut self, (mut transforms, mut ml): Self::SystemData) {
    for transform in (&mut transforms).join() {
      let x = transform.translation().x;
      // println!("{}", x);
      if x < 0.0 {
        shift_map(&mut ml.0[0]);
        shift_map(&mut ml.0[1]);
        transform.set_translation_x(x + (128) as f32);
      }
    }
  }
}

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 8;
const MAP_LENGHT: usize = SCREEN_HEIGHT * SCREEN_WIDTH;

// #[derive(Default, Clone)]
// struct LevelMap([u32; MAP_LENGHT]);
// impl std::fmt::Display for LevelMap {
//   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//     let mut output: String = "".to_owned();
//     for el in self.0.chunks(SCREEN_WIDTH) {
//       output.push_str(&format!("{:?}\n", el));
//     }
//     write!(f, "{}", output)
//   }
// }

fn shift_map(lm: &mut LevelMap) {
  let mut map = lm.clone();
  let mut rng = rand::thread_rng();
  let tile = rng.gen_range(1, 9);
  for (i, el) in &mut map.chunks_mut(SCREEN_WIDTH).enumerate() {
    el.rotate_left(1);
    el[el.len() - 1] = tile;
    el.iter()
      .enumerate()
      .for_each(|(j, x)| lm[i * SCREEN_WIDTH + j] = *x);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_shift_map() {
    // let mut map = LevelMap::default();
    // println!("{:}", &map);
    // shift_map(&mut map);
    // println!("{:}", &map);
    // shift_map(&mut map);
    // println!("{:}", &map);
    // shift_map(&mut map);
    // println!("{:}", &map);
    // shift_map(&mut map);
    // println!("{:}", &map);
    // shift_map(&mut map);
    // println!("{:}", &map);
    // shift_map(&mut map);
    // println!("{:}", &map);
    // shift_map(&mut map);
    // println!("{:}", &map);
  }
}