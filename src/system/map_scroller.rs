use crate::game::ExampleTile;
use amethyst::{
  core::Transform,
  ecs::{Join, ReadStorage, System, WriteStorage},
  tiles::{MortonEncoder, TileMap},
};
pub struct MapScrollerSystem;

impl<'a> System<'a> for MapScrollerSystem {
  type SystemData = (
    ReadStorage<'a, TileMap<ExampleTile, MortonEncoder>>,
    WriteStorage<'a, Transform>,
  );

  fn run(&mut self, (maps, mut transforms): Self::SystemData) {
    for (_, transform) in (&maps, &mut transforms).join() {
      transform.move_left(1.0);
    }
  }
}
