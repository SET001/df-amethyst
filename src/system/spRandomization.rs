use amethyst::{
  core::{math::Vector3, Transform},
  ecs::{Entities, Join, LazyUpdate, Read, System, WriteStorage},
};

use crate::component::{RandomizeSpawnPoint, Velocity};
use rand::Rng;

use crate::states::Moon;

pub struct SpawnPointRandomizationSystem;

impl<'a> System<'a> for SpawnPointRandomizationSystem {
  type SystemData = (
    WriteStorage<'a, Velocity>,
    WriteStorage<'a, Transform>,
    WriteStorage<'a, RandomizeSpawnPoint>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  fn run(
    &mut self,
    (mut velocity, mut transforms, mut randomize, entities, updater): Self::SystemData,
  ) {
    let mut rng = rand::thread_rng();
    for (entity, velocity, transform, random) in
      (&entities, &mut velocity, &mut transforms, &mut randomize).join()
    {
      if transform.translation().x < -500.0 {
        // updater.remove::<Moon>(entity);
      }
    }

    for (entity, velocity, transform, random) in
      (&entities, &mut velocity, &mut transforms, &mut randomize).join()
    {
      if !random.randomized {
        let distance = rng.gen_range(1, 20) as f32;
        let scale = 1.0 / distance;
        let x = rng.gen_range(-500, 500000) as f32;
        let y = rng.gen_range(-500, 500) as f32;
        transform.set_translation_y(y);
        transform.set_translation_x(x);
        transform.set_translation_z(distance * -1.0);
        transform.set_scale(Vector3::new(scale, scale, 1.0));
        velocity.x = -5.0 / distance;
        println!(
          "distance: {}, scale: {}, velocity: {}",
          distance, scale, velocity.x
        );
        random.randomized = true
      }
      // println!("{:?}, {:?}", random, transform);
      // let delta = time.delta_seconds();
      // let delta = 1.0;

      // set_scale
      // transform.set_translation_y(transform.translation().y + delta * velocity.y);
      // transform.set_translation_z(transform.translation().z + delta * velocity.z);
    }
  }
}
// 1->1
// 10->0.1
// 100->0.01
// 1000->0.001
