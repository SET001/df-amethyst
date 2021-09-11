use amethyst::{
  core::{math::Vector3, Transform},
  ecs::{Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage},
};

use crate::component::{Dimensions, RangedScroller, ScrollerItem, Velocity};
use rand::Rng;

pub struct RangedScrollerSystem;

impl<'a> System<'a> for RangedScrollerSystem {
  type SystemData = (
    ReadStorage<'a, ScrollerItem>,
    ReadStorage<'a, Dimensions>,
    ReadStorage<'a, Transform>,
    WriteStorage<'a, RangedScroller>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  fn run(
    &mut self,
    (scrollerItems, dimensions, transforms, mut scrollers, entities, updater): Self::SystemData,
  ) {
    let mut rng = rand::thread_rng();
    for (entity, _item, itemDimensions, itemTransform) in
      (&entities, &scrollerItems, &dimensions, &transforms).join()
    {
      if itemTransform.translation().x < itemDimensions.width / 2.0 * -1.0 {
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
      let mut lastX = 0.0;
      if itemsCount > 0 {
        let lastItemEntity = items.last().unwrap().0;
        lastX = transforms.get(lastItemEntity).unwrap().translation().x
      }
      if lastX < dimension.width {
        // println!("ranged scroller add new item {}, {}", lastX, itemsCount);
        let newItem = entities.create();
        updater.insert(
          newItem,
          Dimensions {
            height: 100.0,
            width: itemWidth,
          },
        );
        let mut itemTransform = Transform::default();
        let x = lastX + rng.gen_range(300, 1300) as f32;
        itemTransform.set_translation_xyz(
          x,
          transform.translation().y - scroller.tiles[0].2 as f32 / 2.0,
          0.0,
        );
        // let scale: f32 = rng.gen_range(30, 100) as f32 / 100.0;
        let scale = 0.5 * scroller.distance;
        // println!("scale is {}", 0.3 * scroller.distance);
        itemTransform.set_scale(Vector3::new(scale, scale, scale));

        updater.insert(newItem, scroller.tiles[0].0.clone());
        updater.insert(newItem, itemTransform);
        updater.insert(newItem, ScrollerItem { scroller: entity });

        updater.insert(
          newItem,
          Velocity {
            x: -1.0 * scroller.distance,
            y: 0.0,
            z: 0.0,
          },
        );
      }
    }
  }
}
