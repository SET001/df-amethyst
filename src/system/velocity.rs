use amethyst::{
  core::Transform,
  ecs::{ParallelRunnable, System, SystemBuilder},
  prelude::IntoQuery,
};

use crate::component::Velocity;

pub struct VelocitySystem;

impl System for VelocitySystem {
  fn build(mut self) -> Box<dyn ParallelRunnable> {
    Box::new(
      SystemBuilder::new("VelocitySystem")
        .with_query(<(&Velocity, &mut Transform)>::query())
        .build(move |_commands, world, (), query| {
          for (velocity, transform) in query.iter_mut(world) {
            let delta = 1.0;
            transform.set_translation_x(transform.translation().x + delta * velocity.x);
            transform.set_translation_y(transform.translation().y + delta * velocity.y);
            transform.set_translation_z(transform.translation().z + delta * velocity.z);
          }
        }),
    )
  }
}
