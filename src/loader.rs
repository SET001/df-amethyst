use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{formats::texture::ImageFormat, Sprite, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::menu::MenuState;

pub struct LoadingState {
    //   /// Tracks loaded assets.
    progress_counter: ProgressCounter,
    //   /// Handle to the player texture.
    pub background_texture_handle: Option<Handle<SpriteSheet>>,
}

impl LoadingState {
    pub fn new() -> LoadingState {
        LoadingState {
            progress_counter: ProgressCounter::new(),
            background_texture_handle: None,
        }
    }

    pub fn load_sprite_sheet(&mut self, texture: Handle<Texture>) -> SpriteSheet {
        let sprite_count = 1; // number of sprites
        let mut sprites = Vec::with_capacity(sprite_count);

        let image_w = 100;
        let image_h = 20;
        let sprite_w = 100;
        let sprite_h = 20;

        // Here we are loading the 5th sprite on the bottom row.
        let offset_x = 0; // 5th sprite * 10 pixel sprite width
        let offset_y = 0; // Second row (1) * 10 pixel sprite height
        let offsets = [0.0; 2]; // Align the sprite with the middle of the entity.

        let sprite = Sprite::from_pixel_values(
            image_w, image_h, sprite_w, sprite_h, offset_x, offset_y, offsets, false, false,
        );
        println!("sprite {:?}", sprite);
        sprites.push(sprite);

        SpriteSheet { texture, sprites }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("loading assets...");
        let loader = &data.world.read_resource::<Loader>();
        let texture_handle = loader.load(
            "logo.png",
            ImageFormat::default(),
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<Texture>>(),
        );

        let loader = &data.world.read_resource::<Loader>();
        // let sprite_sheet = self.load_sprite_sheet(texture_handle);
        // println!("spritesheet {:?}", sprite_sheet);

        let background_texture_handle = loader.load(
            "logo.ron",
            SpriteSheetFormat(texture_handle),
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<SpriteSheet>>(),
        );

        self.background_texture_handle = Some(background_texture_handle);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            let background_texture_handle = self
                .background_texture_handle
                .take()
                .expect("Expected `background_texture_handle` to be loaded.");
            Trans::Switch(Box::new(MenuState {
                background_texture_handle,
            }))
        } else {
            Trans::None
        }
    }
}
