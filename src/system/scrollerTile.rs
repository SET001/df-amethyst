// use crate::component::Scroller;
// use crate::game::ExampleTile;
// use amethyst::window::ScreenDimensions;
// use amethyst::{
//   core::Transform,
//   ecs::{Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
//   tiles::{MortonEncoder, TileMap},
// };

// pub struct ScrollerSystem;

// impl<'a> System<'a> for ScrollerSystem {
//   type SystemData = (
//     ReadExpect<'a, ScreenDimensions>,
//     ReadStorage<'a, Scroller>,
//     WriteStorage<'a, Transform>,
//     Read<'a, LazyUpdate>,
//   );

//   fn run(&mut self, (dimentions, scrollers, mut transforms, updater): Self::SystemData) {
//     for (scroller, transform) in (&scrollers, &mut transforms).join() {
//       transform.move_right(scroller.speed);
//       // println!("{:?}", dimentions.width());
//       // if transform.translation().x < (scroller.tileWidth * -1.0) {
//       //   println!("{:?}", transform.translation().x)
//       // }
//       // if(transform.)
//     }
//   }
// }
