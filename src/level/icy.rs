use rand::Rng;

use crate::level::Level;
const ICE_WALL_TILE: u32 = 0;
const ICE_WALL_AGE_TILE: u32 = 1;
const UPPER_COAST_TILE: u32 = 2;
const BOTTOM_COAST_TILE: u32 = 3;
const BOTTOM_COAST_AGE_TILE: u32 = 4;
const DEFAULT_TILE: u32 = 5;
const BOTTOM_SNOW_TILES: [u32; 9] = [5, 6, 7, 8, 9, 10, 11, 12, 13];

fn get_coordinates_from_index(index: u32, width: u32) -> (u32, u32) {
  return (index % width, index / width);
}

pub struct IcyLevel {
  height: usize,
  width: usize,
  pub map: Vec<u32>,
}

impl IcyLevel {
  pub fn new(width: usize, height: usize) -> IcyLevel {
    IcyLevel {
      height,
      width,
      map: vec![DEFAULT_TILE; height * width],
    }
  }

  pub fn gen_bottom(&mut self) {
    let mut rng = rand::thread_rng();
    self
      .map
      .iter_mut()
      .enumerate()
      .for_each(|(_, x)| *x = BOTTOM_SNOW_TILES[rng.gen_range(0, BOTTOM_SNOW_TILES.len() - 1)]);
  }

  pub fn gen_upper_coast(&mut self) {
    let width = self.width;
    self.map.iter_mut().enumerate().for_each(|(index, tile)| {
      let (_, y) = get_coordinates_from_index(index as u32, width as u32);
      *tile = match y {
        0 => UPPER_COAST_TILE,
        1 => ICE_WALL_TILE,
        2 => ICE_WALL_AGE_TILE,
        _ => *tile,
      }
    });
  }

  pub fn gen_bottom_coast(&mut self) {
    let width = self.width;
    let height = self.height;
    self.map.iter_mut().enumerate().for_each(|(index, tile)| {
      let (_, y) = get_coordinates_from_index(index as u32, width as u32);
      if y == height as u32 - 2 {
        *tile = BOTTOM_COAST_AGE_TILE
      } else if y == height as u32 - 1 {
        *tile = BOTTOM_COAST_TILE
      };
    });
  }
}

impl Level for IcyLevel {
  fn gen_map(&mut self) {
    self.gen_bottom();
    self.gen_upper_coast();
    self.gen_bottom_coast();
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_coordinates_from_index() {
    assert_eq!(get_coordinates_from_index(0, 100), (0, 0));
    assert_eq!(get_coordinates_from_index(20, 100), (20, 0));
    assert_eq!(get_coordinates_from_index(100, 100), (0, 1));
    assert_eq!(get_coordinates_from_index(101, 100), (1, 1));
    assert_eq!(get_coordinates_from_index(315, 100), (15, 3));
  }
  #[test]
  fn test_gen_upper_coast() {
    let mut level = IcyLevel::new(3, 2);
    level.gen_upper_coast();
    assert_eq!(level.map.len(), 6);

    assert_eq!(level.map[0], UPPER_COAST_TILE);
    assert_eq!(level.map[1], UPPER_COAST_TILE);
    assert_eq!(level.map[2], UPPER_COAST_TILE);

    assert_eq!(level.map[3], DEFAULT_TILE);
    assert_eq!(level.map[4], DEFAULT_TILE);
    assert_eq!(level.map[5], DEFAULT_TILE);
  }

  #[test]
  #[ignore]
  fn test_gen_coasts() {}

  #[test]
  #[ignore]
  fn test_gen_bottom_coast() {
    // let map_width: usize = 2;
    // let map = vec![DEFAULT_TILE; map_width * 2];
    // let map_with_coasts = gen_upper_coast(map, map_width);
    // assert_eq!(map_with_coasts[0], DEFAULT_TILE);
    // assert_eq!(map_with_coasts[1], DEFAULT_TILE);
    // assert_eq!(map_with_coasts[2], BOTTOM_COAST_TILE);
  }
  // #[test]
  // #[ignore]
  // fn test_gen_upper_coast() {
  //   // let map_width: usize = 2;
  //   // let map = vec![DEFAULT_TILE; map_width * 2];
  //   // let map_with_coasts = gen_upper_coast(map, map_width);
  //   // println!("{:?}", map_with_coasts);
  // }
}
