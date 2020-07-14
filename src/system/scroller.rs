use amethyst::{
  core::Transform,
  ecs::{Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage},
};

use crate::component::{Dimensions, Scroller, ScrollerItem, Velocity};

pub struct ScrollerSystem;

impl<'a> System<'a> for ScrollerSystem {
  type SystemData = (
    ReadStorage<'a, ScrollerItem>,
    ReadStorage<'a, Dimensions>,
    ReadStorage<'a, Transform>,
    WriteStorage<'a, Scroller>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  fn run(
    &mut self,
    (scrollerItems, dimensions, transforms, mut scrollers, entities, updater): Self::SystemData,
  ) {
    for (entity, _item, itemDimensions, itemTransform) in
      (&entities, &scrollerItems, &dimensions, &transforms).join()
    {
      if itemTransform.translation().x < itemDimensions.width / 2.0 * -1.0 {
        println!("removing bustard!!");
        updater.remove::<ScrollerItem>(entity);
      }
    }
    let items = (&entities, &scrollerItems, &dimensions, &transforms)
      .join()
      .collect::<Vec<_>>();
    for (entity, scroller, dimension, transform) in
      (&entities, &mut scrollers, &dimensions, &transforms).join()
    {
      let itemWidth = scroller.tiles[0].1 as f32;
      let scrollerItems: Vec<(Entity, &ScrollerItem, &Dimensions, &Transform)> = items
        .iter()
        .filter(|&(_itemEntity, item, _dim, _trans)| &(item.scroller) == &entity)
        .cloned()
        .collect();
      let itemsCount = scrollerItems.len();
      if itemWidth * (itemsCount as f32 - 1.0) < dimension.width {
        let newItem = entities.create();
        updater.insert(
          newItem,
          Dimensions {
            height: 100.0,
            width: itemWidth,
          },
        );
        let mut itemTransform = Transform::default();
        let x = (itemsCount as f32) * (itemWidth - 1.0) + itemWidth / 2.0;
        println!("adding new item on {}", x);
        itemTransform.set_translation_xyz(
          x,
          transform.translation().y - scroller.tiles[0].2 as f32 / 2.0,
          0.0,
        );

        updater.insert(newItem, scroller.tiles[0].0.clone());
        updater.insert(newItem, itemTransform);
        updater.insert(newItem, ScrollerItem { scroller: entity });

        updater.insert(
          newItem,
          Velocity {
            x: scroller.speed,
            y: 0.0,
            z: 0.0,
          },
        );
      }
    }
  }
}
