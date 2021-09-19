use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Size {
  pub width: u32,
  pub height: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
  pub map: Size,
  pub show_hud: bool,
  pub show_debug_hud: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppConfig {
  pub tiles: Size,
  pub tile_map: Size,
  pub game: GameConfig,
}
