use amethyst::{
  core::Transform,
  ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage},
};

use crate::component::{Dimensions, Position, Scroller, ScrollerItem, Velocity};

pub struct ScrollerSystem;

impl<'a> System<'a> for ScrollerSystem {
  type SystemData = (
    ReadStorage<'a, Dimensions>,
    ReadStorage<'a, Position>,
    WriteStorage<'a, Scroller>,
    ReadStorage<'a, ScrollerItem>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  fn run(
    &mut self,
    (dimensions, positions, mut scrollers, scrollersItems, entities, updater): Self::SystemData,
  ) {
    for (entity, mut scroller, position, dimension) in
      (&entities, &mut scrollers, &positions, &dimensions).join()
    {
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
            x: -1.0,
            y: 0.0,
            z: 0.0,
          },
        );
        scroller.items.push(newItem);
      }
    }
  }
}
//   println!(
//     "uuid: {}, x: {}, width: {}",
//     scroller.uuid, position.x, dimension.width
//   );
// println!("{:?}", items.len)
// // for () in (&scroller.item)
// // scroller.tiles.len();
// // let b = scroller.tiles.pop().unwrap();
// // let a = b.sprite_sheet.a;
// // let entity = entities.create();
// // let transform = Transform::default();
// // let x = dimensions.width() - 1920.0;
// // transform.set_translation_xyz(-x / 2.0, 0., 0.);
// // updater.insert(entity, transform);
// // updater.insert(entity, Transparent);
// // transform.move_right(scroller.speed);
// // println!("{:?}", dimentions.width());
// // if transform.translation().x < (scroller.tileWidth * -1.0) {
// //   println!("{:?}", transform.translation().x)
// // }
// // if(transform.)
