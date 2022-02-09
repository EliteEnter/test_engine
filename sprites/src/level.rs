use std::{borrow::Borrow, fmt::Debug, ops::DerefMut};

use gm::Point;
use rapier2d::{
    na::Vector2,
    prelude::{ColliderBuilder, RigidBodySet},
};
use rtools::{Rglica, ToRglica};

use crate::{Body, Collider, LevelBase, Sprite, SpriteBase, SpritesDrawer};
pub trait Level: Debug {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn on_key_pressed(&mut self, _: String) {}

    fn on_touch(&mut self, _: Point) {}

    fn convert_touch(&mut self, pos: Point) {
        let mut pos = pos;
        let size = self.drawer().resolution() / self.drawer().scale();

        pos.x -= size.width / 2.0;
        pos.y -= size.height / 2.0;
        pos.y = -pos.y;
        pos /= 10;

        pos += self.player().position();

        self.on_touch(pos)

        // Point pos = touch;
        // auto size = Screen::size / Screen::render_scale();
        // pos.x -= size.width / 2;
        // pos.y -= size.height / 2;
        // pos.y = -pos.y;
        // pos /= 10;
        // pos += _player->position();
        // return pos;
    }

    fn gravity(&self) -> Point {
        let gravity = self.level().gravity.borrow();
        (gravity[0], gravity[1]).into()
    }

    fn set_gravity(&mut self, g: Point) {
        self.level_mut().gravity = Vector2::new(g.x, g.y)
    }

    fn player(&mut self) -> &mut Rglica<Body> {
        debug_assert!(self.level().player.is_ok());
        &mut self.level_mut().player
    }

    fn sprites(&self) -> &[Box<dyn Sprite>] {
        &self.level().sprites
    }

    fn rigid_bodies(&self) -> &RigidBodySet {
        &self.level().sets.rigid_body
    }

    fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet {
        &mut self.level_mut().sets.rigid_body
    }

    fn add_body(&mut self, sprite: SpriteBase) -> Rglica<Body> {
        Body::make(sprite, self.level_mut())
    }

    fn add_sprite(&mut self, sprite: SpriteBase) {
        self.level_mut().sprites.push(Box::new(sprite))
    }

    fn add_wall(&mut self, sprite: SpriteBase) -> Rglica<Collider> {
        let collider = ColliderBuilder::cuboid(sprite.size().width, sprite.size().height)
            .translation(Vector2::new(sprite.position().x, sprite.position().y))
            .build();
        self.level_mut().sets.collider.insert(collider);
        let boxed = Box::<Collider>::new(sprite.into());
        let wall = boxed.to_rglica();
        self.level_mut().sprites.push(boxed);
        wall
    }

    fn set_camera_rotation(&mut self, angle: f32) {
        self.drawer().set_camera_rotation(angle)
    }

    fn level(&self) -> &LevelBase;
    fn level_mut(&mut self) -> &mut LevelBase;

    fn drawer(&mut self) -> &mut dyn SpritesDrawer {
        self.level_mut().drawer.deref_mut()
    }

    fn set_drawer(&mut self, drawer: Rglica<dyn SpritesDrawer>) {
        self.level_mut().drawer = drawer
    }
}
