use amethyst::{
  core::Transform,
  ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage},
};

use crate::component::{Dimensions, Scroller, Velocity};

pub struct ScrollerSystem;

impl<'a> System<'a> for ScrollerSystem {
  type SystemData = (
    ReadStorage<'a, Dimensions>,
    WriteStorage<'a, Scroller>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  fn run(&mut self, (dimensions, mut scrollers, entities, updater): Self::SystemData) {
    for (scroller, dimension) in (&mut scrollers, &dimensions).join() {
      let itemsWidth: f32 = scroller
        .items
        .iter()
        .map(|&item| dimensions.get(item).unwrap().width)
        .sum();

      if itemsWidth < dimension.width {
        let newItem = entities.create();
        updater.insert(
          newItem,
          Dimensions {
            height: 100.0,
            width: 200.0,
          },
        );
        updater.insert(newItem, scroller.tiles[0].clone());
        updater.insert(newItem, Transform::default());
        updater.insert(
          newItem,
          Velocity {
            x: scroller.speed,
            y: 0.0,
            z: 0.0,
          },
        );
        scroller.items.push(newItem);
      }
    }
  }
}
