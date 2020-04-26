use amethyst::assets::AssetStorage;
use amethyst::core::transform::Transform;
use amethyst::window::ScreenDimensions;
use amethyst::{
    assets::Handle,
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        SpriteRender, SpriteSheet, Texture, Transparent,
    },
};
#[derive(Clone)]
pub struct MenuState {
    pub background_texture_handle: Handle<SpriteSheet>,
}

impl SimpleState for MenuState {
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        _event: StateEvent,
    ) -> SimpleTrans {
        Trans::None
    }
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        println!("menu opened");
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        init_camera(data.world, &dimensions);
        initialize_sprite(&mut data.world, self.background_texture_handle.clone());
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 10.0);
    let camera = Camera::from(Projection::orthographic(
        -dimensions.width() / 2.,
        dimensions.width() / 2.,
        -dimensions.height() / 2.,
        dimensions.height() / 2.,
        0.,
        200.,
    ));
    // transform.set_translation_xyz(0.0, 0.0, -1.0);
    // let camera = Camera::standard_2d(dimensions.width(), dimensions.height());

    println!("camera_transform: {:?}", transform.translation());
    world.create_entity().with(camera).with(transform).build();
}

fn initialize_sprite(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    // Move the sprite to the middle of the window
    let mut sprite_transform = Transform::default();
    // sprite_transform.set_translation_xyz(0., 0., 10.0);
    sprite_transform.set_translation_xyz(width / 2., height / 2., 1.);

    println!("sprite_transform: {:?}", sprite_transform.translation());
    // sprite_transform.set_translation_xyz(0., 0., 1.);
    {
        let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();
        if let Some(sprite_sheet) = sprite_sheet_assets.get(&sprite_sheet_handle) {
            // println!("spritesheet {:?}", sprite_sheet);
            println!("spritesheet is some");

            let texture_assets = world.read_resource::<AssetStorage<Texture>>();
            if texture_assets.get(&sprite_sheet.texture).is_some() {
                println!("texture is some");
            }
        }
    }

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // First sprite
    };
    world
        .create_entity()
        .with(sprite_render)
        .with(sprite_transform)
        .with(Transparent)
        .build();
}
