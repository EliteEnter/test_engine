use std::ops::Deref;

use gm::flat::{Point, Size};
use level::{LevelManager, Sprite};
use ui::UIManager;
use wgpu::RenderPass;
use wgpu_wrapper::WGPUDrawer;

pub(crate) struct TELevel;

impl TELevel {
    pub(crate) fn update() {
        LevelManager::update();
    }

    pub(crate) fn draw<'a>(pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer) {
        if LevelManager::no_level() {
            return;
        }
        let resolution = UIManager::window_size();
        for sprite in LevelManager::level_mut().sprites() {
            Self::draw_sprite(sprite.deref(), pass, drawer, resolution)
        }
    }

    fn draw_sprite<'a>(
        sprite: &dyn Sprite,
        pass: &mut RenderPass<'a>,
        drawer: &'a WGPUDrawer,
        resolution: Size,
    ) {
        drawer.sprite_drawer.draw(
            pass,
            sprite.size(),
            sprite.position(),
            sprite.rotation(),
            1.0,
            0.0,
            Point::default(),
            resolution,
            *sprite.color(),
        );
    }
}

// pub struct TESpritesDrawer {
//     scale:           f32,
//     resolution:      IntSize,
//     camera_position: Point,
// }
//
// impl TESpritesDrawer {
//     pub fn new() -> Box<Self> {
//         let mut new = Self {
//             scale:           0.,
//             resolution:      Default::default(),
//             camera_position: Default::default(),
//         };
//         new.set_scale(1.);
//         Box::new(new)
//     }
// }
//
// impl SpritesDrawer for TESpritesDrawer {
//     fn scale(&self) -> f32 {
//         self.scale
//     }
//
//     fn set_scale(&mut self, scale: f32) {
//         self.scale = scale;
//         SpriteShaders::sprite().enable().set_scale(scale.into_f32());
//         SpriteShaders::textured_sprite().enable().set_scale(scale);
//     }
//
//     fn resolution(&self) -> IntSize {
//         self.resolution
//     }
//
//     fn set_resolution(&mut self, size: IntSize) {
//         self.resolution = size;
//         SpriteShaders::sprite().enable().set_resolution(size.into());
//         SpriteShaders::textured_sprite().enable().set_resolution(size.
// into());     }
//
//     fn set_camera_rotation(&self, angle: f32) {
//         let angle = angle + std::f32::consts::PI / 2.;
//         SpriteShaders::sprite().enable().set_camera_rotation(angle);
//         SpriteShaders::textured_sprite().enable().set_camera_rotation(angle);
//     }
//
//     fn camera_position(&self) -> Point {
//         self.camera_position
//     }
//
//     fn set_camera_position(&mut self, pos: Point) {
//         self.camera_position = pos;
//         SpriteShaders::sprite().enable().set_camera_position(pos);
//         SpriteShaders::textured_sprite().enable().set_camera_position(pos);
//     }
//
//     fn draw(&self, sprite: &dyn Sprite) {
//         let buffers = Buffers::get();
//         // let ass = Assets::get();
//
//         let (shader, buffer) = if sprite.image().is_ok() {
//             (SpriteShaders::textured_sprite(), &buffers.full_image)
//         } else {
//             (SpriteShaders::sprite(), &buffers.full)
//         };
//
//         shader
//             .enable()
//             .set_selected(sprite.is_selected())
//             .set_size(sprite.size())
//             .set_position(sprite.position())
//             .set_rotation(sprite.rotation());
//
//         if let Some(image) = sprite.image().get() {
//             shader.set_flipped(image.flipped).set_flipped_y(image.flipped_y);
//             image.bind();
//         } else {
//             shader.set_color(sprite.color());
//         }
//
//         buffer.draw();
//     }
// }