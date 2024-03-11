use gm::{
    flat::{Point, PointsPath},
    Color,
};
use refs::Weak;
use ui::{view, Sub, Touch, ViewFrame, ViewSetup, ViewTouch};
use vents::Event;
use wgpu_wrapper::PolygonMode;

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::DrawingView;

const SIZE: f32 = 200.0;
const OUTLINE_WIDTH: f32 = 10.0;
const STICK_VIEW_SIZE: f32 = SIZE / 2.0;
const PRECISION: u16 = 50;

#[view]
pub struct StickView {
    background:      Sub<DrawingView>,
    direction_stick: Sub<DrawingView>,
    pub on_change:   Event<Point>,
    pub flaccid:     bool,
}

impl StickView {
    fn on_touch_moved(&mut self, touch: Point) {
        let max_length = self.frame().size.height / 2.0;
        let center = self.frame().size.center();

        let vector = (touch - center).trimmed(max_length);

        let frame = *self.frame();

        self.direction_stick.set_center(vector + frame.size.center());

        self.on_change.trigger(vector * 0.1);
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            if self.flaccid {
                return;
            }
            let frame = *self.frame();
            self.direction_stick.set_center(frame.size.center());
            self.on_change.trigger(Point::default());
        } else {
            self.on_touch_moved(touch.position);
        }
    }
}

impl ViewSetup for StickView {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.touch.all.val(move |touch| {
            self.on_touch(&touch);
        });

        self.set_frame((SIZE, SIZE));

        self.background.set_frame((SIZE, SIZE));

        let frame = *self.frame();
        self.background.add_path(
            PointsPath::circle_triangles_with(frame.size.center(), frame.size.width / 2.0, PRECISION),
            Color::BLACK,
            PolygonMode::Fill,
        );

        self.background.add_path(
            PointsPath::circle_triangles_with(
                frame.size.center(),
                (frame.size.width - OUTLINE_WIDTH) / 2.0,
                PRECISION,
            ),
            Color::WHITE,
            PolygonMode::Fill,
        );

        let center = self.frame().size.center();

        self.direction_stick
            .set_frame((STICK_VIEW_SIZE, STICK_VIEW_SIZE))
            .set_center(center);

        let stick_center = self.direction_stick.frame().size.center();

        self.direction_stick
            .add_path(
                PointsPath::circle_triangles_with(stick_center, STICK_VIEW_SIZE / 2.0, PRECISION),
                Color::BLACK,
                PolygonMode::Fill,
            )
            .add_path(
                PointsPath::circle_triangles_with(
                    stick_center,
                    (STICK_VIEW_SIZE - OUTLINE_WIDTH) / 2.0,
                    PRECISION,
                ),
                Color::LIGHT_GRAY,
                PolygonMode::Fill,
            );
    }
}