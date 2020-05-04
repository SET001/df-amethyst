use crate::component::Scroller;
use crate::game::ExampleTile;
use amethyst::{
  core::Transform,
  ecs::{Join, ReadStorage, System, WriteStorage},
  tiles::{MortonEncoder, TileMap},
};

pub struct MapScrollerSystem;

impl<'a> System<'a> for MapScrollerSystem {
  type SystemData = (ReadStorage<'a, Scroller>, WriteStorage<'a, Transform>);

  fn run(&mut self, (scrollers, mut transforms): Self::SystemData) {
    for (scroller, transform) in (&scrollers, &mut transforms).join() {
      transform.move_left(scroller.speed);
    }
  }
}
