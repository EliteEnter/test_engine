// use std::ops::DerefMut;
//
// use manage::data_manager::DataManager;
// use old_engine::{
//     audio::Sound,
//     gm::{
//         flat::{Point, Shape},
//         Color,
//     },
// };
// use ui::refs::{weak_from_ref, Weak};
//
// #[derive(Default)]
// pub struct TestGameLevel {
//     base:            LevelBase,
//     selected_sprite: Option<Weak<dyn Sprite>>,
//     collision_sound: Weak<Sound>,
// }
//
// impl TestGameLevel {
//     fn on_touch(&mut self, pos: Point) {
//         if let Some(mut sprite) = self.sprite_at(pos) {
//             sprite.set_selected(true);
//             self.base_mut().on_sprite_selected.trigger(sprite);
//             if let Some(mut old) = self.selected_sprite {
//                 old.set_selected(false);
//             }
//             self.selected_sprite = sprite.into();
//             return;
//         }
//
//         if let Some(mut sprite) = self.selected_sprite {
//             sprite.set_selected(false);
//             self.selected_sprite = None;
//             self.base_mut().on_sprite_selected.trigger(Weak::default());
//         }
//     }
// }
//
// impl Level for TestGameLevel {
//     fn setup(&mut self) {
//         let drawn = GlImage::render("test_draw", (100, 100), |image| {
//             GLWrapper::set_clear_color(Color::GREEN);
//             GLWrapper::clear();
//             GLWrapper::scissor((5, 5, 20, 20), || {
//                 GLWrapper::set_clear_color(Color::TURQUOISE);
//                 GLWrapper::clear();
//             });
//             GLWrapper::set_clear_color(Color::GRAY);
//             image.channels = 1;
//         });
//
//         let square = GlImage::get("square.png");
//
//         self.add_rect((30, 30, 40, 25)).set_image(drawn);
//
//         self.add_sprite::<Wall>(Shape::Rect((100, 5).into()), (0, 0))
//             .set_image(render_text("oo spolokolkok",
// GlFont::helvetica().deref_mut(), 64));         self.
// add_sprite::<Wall>(Shape::Rect((5, 100).into()), (60, 0)).set_image(square);
//         self.add_sprite::<Wall>(Shape::Rect((5, 100).into()), (-60, 0))
//             .set_image(square);
//
//         self.add_sprite::<Body>(Shape::triangle((-10, -10), (10, -10), (-10,
// 10)), (0, 50))             .set_image("triangle.png");
//
//         let concave_points: Vec<Point> = vec![
//             (5, -5).into(),
//             (-10, -10).into(),
//             (10, -10).into(),
//             (10, 10).into(),
//         ];
//
//         self.add_sprite::<Body>(Shape::Polygon(concave_points), (0, 100))
//             .set_image("triangle.png");
//
//         for i in 0..500 {
//             self.add_sprite::<Body>(Shape::Rect((0.5, 0.5).into()), (0.1 * i
// as f32, i * 2))                 .set_image(square);
//         }
//
//         let mut player: Weak<Player> = self.add_sprite(Shape::Rect((2,
// 2).into()), (0, 5));         self.base_mut().player = player;
//         player.set_image("frisk.png").enable_collision_detection();
//         player.weapon.set_image("ak.png");
//         let mut this = weak_from_ref(self);
//         player.on_collision.sub(move || {
//             this.collision_sound.play();
//         });
//
//         self.collision_sound = Sound::get("pek.wav");
//
//         self.base.on_tap.val(move |pos| this.on_touch(pos));
//     }
//
//     fn update(&mut self) {
//         let pos = self.player().position();
//         self.set_camera_position(pos);
//     }
//
//     fn base(&self) -> &LevelBase {
//         &self.base
//     }
//
//     fn base_mut(&mut self) -> &mut LevelBase {
//         &mut self.base
//     }
//
//     fn weak_level(&self) -> Weak<dyn Level> {
//         weak_from_ref(self as &dyn Level)
//     }
// }