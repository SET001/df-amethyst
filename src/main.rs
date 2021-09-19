#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod bundle;
mod component;
mod config;
mod hud;
mod states;
mod system;
mod tiles;

use amethyst::{
  assets::LoaderBundle,
  core::transform::TransformBundle,
  input::InputBundle,
  prelude::*,
  renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    rendy::hal::command::ClearColor,
    types::DefaultBackend,
    RenderingBundle,
  },
  tiles::{MortonEncoder, RenderTiles2D},
  ui::{RenderUi, UiBundle},
  utils::{application_root_dir, fps_counter::FpsCounterBundle},
};
use bundle::DfBundle;
use states::{GameState, LoadingState};
use tiles::{IcyTile, SteppeTile};

fn main() -> amethyst::Result<()> {
  amethyst::Logger::from_config(Default::default())
    .level_for("amethyst_tiles", log::LevelFilter::Debug)
    .start();

  let app_root = application_root_dir()?;
  let assets_dir = app_root.join("assets");
  let config_dir = app_root.join("config");
  let display_config_path = config_dir.join("display.ron");

  let mut dispatcher = DispatcherBuilder::default();
  dispatcher.add_bundle(LoaderBundle);
  dispatcher.add_bundle(TransformBundle);
  dispatcher.add_bundle(InputBundle::new());
  dispatcher.add_bundle(UiBundle::<u32>::default());
  dispatcher.add_bundle(DfBundle::from_config_path(config_dir.join("game.ron"))?);
  dispatcher.add_bundle(FpsCounterBundle::default());
  dispatcher.add_bundle(
    RenderingBundle::<DefaultBackend>::new()
      .with_plugin(
        RenderToWindow::from_config_path(display_config_path)?.with_clear(ClearColor {
          float32: [0.0, 0.0, 0.0, 1.0],
        }),
      )
      .with_plugin(RenderUi::default())
      .with_plugin(RenderFlat2D::default())
      .with_plugin(RenderTiles2D::<IcyTile, MortonEncoder>::default())
      .with_plugin(RenderTiles2D::<SteppeTile, MortonEncoder>::default()),
  );
  let game = Application::new(assets_dir, LoadingState::new(), dispatcher)?;
  game.run();
  Ok(())
}
