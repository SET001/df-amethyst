pub struct IcyMapGeneratorSystem;

impl<'a> System<'a> for IcyMapGeneratorSystem {
  type SystemData = (
    ReadStorage<'a, ScrollerItem>,
    ReadStorage<'a, Dimensions>,
    ReadStorage<'a, Transform>,
    WriteStorage<'a, RangedScroller>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );
}
