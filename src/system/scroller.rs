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
    let itemWidth = 232.0;
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

    for (entity, scroller, dimension) in (&entities, &mut scrollers, &dimensions).join() {
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
        let mut transform = Transform::default();
        let x = (itemsCount as f32) * (itemWidth) + itemWidth / 2.0;
        println!("adding new item on {}", x);
        transform.set_translation_xyz(x, 0.0, 0.0);

        updater.insert(newItem, scroller.tiles[0].clone());
        updater.insert(newItem, transform);
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
    // for (itemEntity, item, itemDimensions, itemTransform) in items
    // {
    //   if item.scroller == entity {

    //   }
    //   // if itemTransform.translation().x < itemDimensions.width / 2.0 * -1.0 {
    //   //   updater.remove::<ScrollerItem>(entity);
    //   // }
    // }
    // }
    //   let itemsCount = scroller.items.len();
    //   // let itemsWidth: f32 = scroller
    //   //   .items
    //   //   .iter()
    //   //   .map(|&item| dimensions.get(item).unwrap().width)
    //   //   .sum();

    //   let itemsToRemove: Vec<Entity> = scroller
    //     .items
    //     .iter()
    //     .filter(|&item| transforms.get(*item).unwrap().translation().x < itemWidth / 2.0 * -1.0)
    //     .cloned()
    //     .collect();

    //   println!(
    //     "items in scroller {}, items to remove: {}",
    //     itemsCount,
    //     itemsToRemove.len()
    //   );
    //   // scroller.items = scroller
    //   //   .items
    //   //   .iter()
    //   //   .filter(|&item| transforms.get(*item).unwrap().translation().x < itemWidth * -1.0)
    //   //   .cloned()
    //   //   .collect();
    //   if itemWidth * (itemsCount as f32) < dimension.width {
    //     let newItem = entities.create();
    //     updater.insert(
    //       newItem,
    //       Dimensions {
    //         height: 100.0,
    //         width: itemWidth,
    //       },
    //     );
    //     let mut transform = Transform::default();
    //     transform.set_translation_xyz((itemsCount as f32) * itemWidth, 0.0, 0.0);

    //     updater.insert(newItem, scroller.tiles[0].clone());
    //     updater.insert(newItem, transform);

    //     updater.insert(
    //       newItem,
    //       Velocity {
    //         x: scroller.speed,
    //         y: 0.0,
    //         z: 0.0,
    //       },
    //     );
    //     scroller.items.push(newItem);
    //   }
    // }
  }
}
