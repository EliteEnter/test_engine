use test_engine::{
    after,
    level::LevelManager,
    refs::Weak,
    ui::{
        view,
        Anchor::{Size, Top, X},
        Button, Color, Container, Point, PositionView, UIManager, ViewCallbacks, ViewData, ViewFrame,
        ViewSetup, ViewSubviews,
    },
    RenderPass, SpriteView, VertexBuffer, WGPUApp,
};

use crate::interface::test_game_view::TestGameView;

#[view]
pub struct PolygonView {
    points: VertexBuffer,
    views:  Vec<Weak<PositionView>>,
    #[init]
    add:    Button,
    print:  Button,
    center: Container,
}

impl ViewSetup for PolygonView {
    fn setup(mut self: Weak<Self>) {
        LevelManager::stop_level();

        self.add_transition::<Self, TestGameView>()
            .set_text("Back")
            .place()
            .t(200)
            .l(10)
            .size(100, 50);

        self.add.set_text("Add").place().t(200).r(10).size(100, 50);
        self.add.on_tap(move || {
            self.add_point((0, 0).into());
        });

        self.print.set_text("Print");
        self.print.place().anchor(Top, self.add, 10).same([Size, X], self.add);
        self.print.on_tap(move || {
            dbg!(&self.points);
        });

        self.center.set_color(Color::WHITE).place().size(5, 5).center();

        after(0.1, move || self.add_first_points());
    }
}

impl PolygonView {
    fn add_point(mut self: Weak<Self>, pos: Point) {
        let mut view = self.add_view::<PositionView>();
        view.set_position(pos);
        view.tag = self.points.vertices.len();
        view.additional_label = format!("{}:", self.points.vertices.len()).into();
        let pos = LevelManager::convert_touch(pos);
        self.points.vertices.push(pos);

        view.moved.val(self, move |new_pos| {
            self.points.vertices[view.tag] = LevelManager::convert_touch(new_pos);
        });
    }

    fn add_first_points(self: Weak<Self>) {
        self.add_point((200, 200).into());
        self.add_point((200, 500).into());
        self.add_point((500, 200).into());
    }
}

impl ViewCallbacks for PolygonView {
    fn render(&self, pass: &mut RenderPass) {
        let drawer = WGPUApp::drawer();

        drawer.polygon.clear();

        drawer.polygon.add(&self.points, (0, 0).into(), Color::GREEN, 0.0);

        drawer.polygon.draw(
            pass,
            SpriteView {
                camera_pos:      Point::default(),
                resolution:      UIManager::resolution(),
                camera_rotation: 0.0,
                scale:           1.0,
            },
        );
    }
}