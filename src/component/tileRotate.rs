use amethyst::ecs::{Component, VecStorage};

#[derive(Default, Clone)]
pub struct TileRotate {
  pub rotaionCycle: u32,
}

impl Component for TileRotate {
  type Storage = VecStorage<Self>;
}
