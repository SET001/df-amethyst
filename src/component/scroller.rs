use amethyst::ecs::Entity;

use uuid::Uuid;

use amethyst::{
  ecs::{Component, VecStorage},
  renderer::SpriteRender,
};

#[derive(Default, Clone)]
pub struct Scroller {
  pub speed: f32,
  pub tiles: Vec<(SpriteRender, u32, u32)>,
  pub uuid: String,
  pub items: Vec<Entity>,
}

impl Scroller {
  pub fn new(tiles: Vec<(SpriteRender, u32, u32)>, speed: f32) -> Scroller {
    Scroller {
      tiles,
      speed,
      uuid: Uuid::new_v4().to_string(),
      items: vec![],
    }
  }
}

impl Component for Scroller {
  type Storage = VecStorage<Self>;
}

#[derive(Default, Clone)]
pub struct RangedScroller {
  pub speed: f32,
  pub tiles: Vec<(SpriteRender, u32, u32)>,
  pub uuid: String,
  pub items: Vec<Entity>,
}

impl RangedScroller {
  pub fn new(tiles: Vec<(SpriteRender, u32, u32)>, speed: f32) -> Scroller {
    Scroller {
      tiles,
      speed,
      uuid: Uuid::new_v4().to_string(),
      items: vec![],
    }
  }
}

impl Component for RangedScroller {
  type Storage = VecStorage<Self>;
}

#[derive(Clone)]
pub struct ScrollerItem {
  pub scroller: Entity,
}

impl Component for ScrollerItem {
  type Storage = VecStorage<Self>;
}
