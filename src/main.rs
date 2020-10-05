#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use amethyst::{
  assets::PrefabLoaderSystemDesc,
  core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
  input::{InputBundle, StringBindings},
  prelude::*,
  renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    rendy::mesh::{Normal, TexCoord},
    types::DefaultBackend,
    RenderingBundle,
  },
  // tiles::{MortonEncoder, RenderTiles2D},
  ui::{RenderUi, UiBundle},
  utils::{application_root_dir, fps_counter::FpsCounterBundle, scene::BasicScenePrefab},
};

// use log::info;

mod component;
mod states;
mod system;

use crate::states::Moon;
use crate::states::{LoadingState, PrefabsTest};

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;
  let assets_dir = app_root.join("assets/");
  let display_config_path = app_root.join("config/display.ron");

  let game_data = GameDataBuilder::default()
    .with_system_desc(
      PrefabLoaderSystemDesc::<Moon>::default(),
      "scene_loader",
      &[],
    )
    // .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
    //     "sprite_animation_control",
    //     "sprite_sampler_interpolation",
    // ))?
    .with_bundle(TransformBundle::new())?
    .with(system::ScrollerSystem, "scrolling_system", &[])
    .with(system::VelocitySystem, "velocity_system", &[])
    .with(
      system::SpawnPointRandomizationSystem,
      "sp randomization system",
      &[],
    )
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
          RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.34, 0.36, 0.52, 1.0]),
        )
        .with_plugin(RenderFlat2D::default()),
    )?;

  let mut game = Application::new(assets_dir, PrefabsTest::default(), game_data)?;
  game.run();

  Ok(())
}
