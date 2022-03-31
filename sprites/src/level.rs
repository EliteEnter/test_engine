use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use gm::Point;
use rapier2d::{
    na::Vector2,
    prelude::{ColliderSet, RigidBodySet},
};
use rtools::{Rglica, ToRglica};

use crate::{Body, LevelBase, Player, Sprite, SpriteBase, SpritesDrawer, Wall};

pub trait Level: Debug {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn on_key_pressed(&mut self, _: String) {}

    fn cursor_position(&self) -> Point {
        self.level().cursor_position
    }

    fn set_cursor_position(&mut self, pos: Point) {
        self.level_mut().cursor_position = self.convert_touch(pos)
    }

    fn add_touch(&mut self, pos: Point) {
        let pos = self.convert_touch(pos);
        self.level_mut().on_tap.trigger(pos);
    }

    fn convert_touch(&self, pos: Point) -> Point {
        let mut pos = pos;
        let size = self.drawer().resolution();

        pos.x -= size.width / 2.0;
        pos.y -= size.height / 2.0;
        pos.y = -pos.y;
        pos /= 10;

        pos *= 2;
        pos /= self.drawer().scale();

        pos += self.player().position();

        pos
    }

    fn sprite_at(&self, point: Point) -> Option<Rglica<dyn Sprite>> {
        for bx in self.sprites() {
            if bx.contains(point) {
                return bx.to_rglica().into();
            }
        }
        None
    }

    fn scale(&self) -> f32 {
        self.drawer().scale()
    }

    fn set_scale(&mut self, scale: f32) {
        self.drawer_mut().set_scale(scale)
    }

    fn gravity(&self) -> Point {
        let gravity = self.level().gravity.borrow();
        (gravity[0], gravity[1]).into()
    }

    fn set_gravity(&mut self, g: Point) {
        self.level_mut().gravity = Vector2::new(g.x, g.y)
    }

    fn player(&self) -> Rglica<Player> {
        debug_assert!(self.level().player.is_ok());
        self.level().player.to_rglica()
    }

    fn sprites(&self) -> &[Box<dyn Sprite>] {
        &self.level().sprites
    }

    fn sprites_mut(&mut self) -> &mut [Box<dyn Sprite>] {
        &mut self.level_mut().sprites
    }

    fn rigid_bodies(&self) -> &RigidBodySet {
        &self.level().sets.rigid_body
    }

    fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet {
        &mut self.level_mut().sets.rigid_body
    }

    fn colliders(&self) -> &ColliderSet {
        &self.level().sets.collider
    }

    fn colliders_mut(&mut self) -> &mut ColliderSet {
        &mut self.level_mut().sets.collider
    }

    fn add_body(&mut self, sprite: SpriteBase) -> Rglica<Body> {
        let body = Box::new(Body::make(sprite, self.level_mut()));
        let result = body.to_rglica();
        self.level_mut().sprites.push(body);
        result
    }

    fn add_wall(&mut self, sprite: SpriteBase) -> Rglica<Wall> {
        Wall::make(sprite, self.level_mut())
    }

    fn set_camera_rotation(&mut self, angle: f32) {
        self.drawer().set_camera_rotation(angle)
    }

    fn remove(&mut self, sprite: u64) {
        self.level_mut().remove(sprite)
    }

    fn level(&self) -> &LevelBase;
    fn level_mut(&mut self) -> &mut LevelBase;

    fn drawer(&self) -> &dyn SpritesDrawer {
        self.level().drawer.deref()
    }

    fn drawer_mut(&mut self) -> &mut dyn SpritesDrawer {
        self.level_mut().drawer.deref_mut()
    }

    fn set_drawer(&mut self, drawer: Rglica<dyn SpritesDrawer>) {
        self.level_mut().drawer = drawer
    }
}
