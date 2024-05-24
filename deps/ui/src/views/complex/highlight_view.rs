use gm::{flat::Point, Apply, Color};
use refs::Weak;
use ui_proc::view;

use crate::{Container, Sub, ViewData, ViewFrame, ViewSetup, ViewSubviews};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct HighlightView {
    t: Sub<Container>,
    b: Sub<Container>,
    l: Sub<Container>,
    r: Sub<Container>,
}

impl HighlightView {}

impl ViewSetup for HighlightView {
    fn setup(mut self: Weak<Self>) {
        const WIDTH: f32 = 40.0;

        self.t.place().lrt(0).h(WIDTH);
        self.b.place().lrb(0).h(WIDTH);
        self.l.place().t(0).l(0).b(0).w(WIDTH);
        self.r.place().t(0).r(0).b(0).w(WIDTH);

        self.outline(Color::BLACK);
    }
}

impl HighlightView {
    pub fn set(&mut self, pos: impl Into<Point>, expected: Color, actual: Color) {
        self.set_size((150, 150));
        self.set_center(pos);
        [self.t, self.b, self.l].apply(|mut v| {
            v.set_color(expected);
        });
        self.r.set_color(actual);
    }
}
