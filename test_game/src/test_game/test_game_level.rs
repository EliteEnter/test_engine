use rtools::{Rglica, ToRglica};
use test_engine::{
    assets::Assets,
    gm::Point,
    sprites::{add_sprite, Body, Control, Player, Wall},
    Level, LevelBase, Sprite,
};

#[derive(Default, Debug)]
pub struct TestGameLevel {
    base:            LevelBase,
    selected_sprite: Option<Rglica<dyn Sprite>>,
    pub player:      Rglica<Player>,
}

impl TestGameLevel {
    fn on_touch(&mut self, pos: Point) {
        if let Some(mut sprite) = self.sprite_at(pos) {
            sprite.set_selected(true);
            self.base_mut().on_sprite_selected.trigger(sprite);
            if let Some(mut old) = self.selected_sprite {
                old.set_selected(false);
            }
            self.selected_sprite = sprite.into();
            return;
        }

        if let Some(mut sprite) = self.selected_sprite {
            sprite.set_selected(false);
            self.selected_sprite = None;
            self.base_mut().on_sprite_selected.trigger(Rglica::default());
        }
    }
}

impl Level for TestGameLevel {
    fn setup(&mut self) {
        self.player = add_sprite((0, 5, 2, 2), self);
        self.player.set_image(Assets::image("frisk.png"));

        self.player.weapon.set_image(Assets::image("frisk.png"));

        let square = Assets::image("square.png");

        add_sprite::<Wall>((0, 0, 100, 1), self).set_image(square.clone());
        add_sprite::<Wall>((20, 0, 1, 100), self).set_image(square.clone());
        add_sprite::<Wall>((-20, 0, 1, 100), self).set_image(square);

        for i in 0..50 {
            let body = Body::make((0.1 * i as f32, i * 2, 0.5, 0.5).into(), self.rglica());
            self.add_sprite(body);
        }

        let mut this = self.to_rglica();
        self.base.on_tap.subscribe(move |pos| this.on_touch(pos));
    }

    fn update(&mut self) {
        let pos = self.player.position();
        self.drawer_mut().set_camera_position(pos);
    }

    fn on_key_pressed(&mut self, key: String) {
        self.player.move_by_key(key)
    }

    fn base(&self) -> &LevelBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }

    fn rglica(&self) -> Rglica<dyn Level> {
        (self as &dyn Level).to_rglica()
    }
}