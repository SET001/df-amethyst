use amethyst::{
  core::Transform,
  ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::component::Velocity;

pub struct VelocitySystem;

impl<'a> System<'a> for VelocitySystem {
  type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Transform>);

  fn run(&mut self, (velocities, mut transforms): Self::SystemData) {
    for (velocity, transform) in (&velocities, &mut transforms).join() {
      // let delta = time.delta_seconds();
      let delta = 1.0;
      transform.set_translation_x(transform.translation().x + delta * velocity.x);
      transform.set_translation_y(transform.translation().y + delta * velocity.y);
      transform.set_translation_z(transform.translation().z + delta * velocity.z);
    }
  }
}
