use amethyst::{
  core::Transform,
  ecs::{ParallelRunnable, System, SystemBuilder},
  prelude::IntoQuery,
};

use crate::{component::TileRotate, config::AppConfig};

pub struct TileRotateSystem;

impl System for TileRotateSystem {
  fn build(self) -> Box<dyn ParallelRunnable> {
    Box::new(
      SystemBuilder::new("TileRotateSystem")
        .read_resource::<AppConfig>()
        .with_query(<(&TileRotate, &mut Transform)>::query())
        .build(move |_commands, world, config, query| {
          let tile_width = config.tiles.width;
          for (_, transform) in query.iter_mut(world) {
            let x = transform.translation().x;
            if x == (tile_width as i32 * -1) as f32 {
              transform.set_translation_x(x + tile_width as f32);
            }
          }
        }),
    )
  }
}
