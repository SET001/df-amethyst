use amethyst::{
  assets::PrefabLoaderSystemDesc,
  core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
  input::{InputBundle, StringBindings},
  prelude::*,
  renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    rendy::mesh::{Normal, Position, TexCoord},
    types::DefaultBackend,
    RenderingBundle,
  },
  tiles::{MortonEncoder, RenderTiles2D},
  ui::{RenderUi, UiBundle},
  utils::{
    application_root_dir,
    fps_counter::{FpsCounter, FpsCounterBundle},
    scene::BasicScenePrefab,
  },
};

use log::info;

mod game;
mod loader;
mod menu;

use crate::game::ExampleTile;
use crate::loader::LoadingState;

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;
  let display_config_path = app_root.join("config").join("display.ron");

  let game_data = GameDataBuilder::default()
    .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
    .with_bundle(TransformBundle::new())?
    .with_bundle(InputBundle::<StringBindings>::new())?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(FpsCounterBundle::default())?
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
          RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default())
        .with_plugin(RenderTiles2D::<ExampleTile, MortonEncoder>::default()),
    )?;

  let assets_dir = app_root.join("assets");
  let mut game = Application::build(assets_dir, LoadingState::new())?
    .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
    .build(game_data)?;
  game.run();

  Ok(())
}
