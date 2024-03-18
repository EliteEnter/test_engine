use gm::{converter::Converter, Color, IntoF32};
use refs::Weak;
use ui::{view, Sub, Touch, ViewCallbacks, ViewFrame, ViewSetup, ViewTouch};
use vents::Event;

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::CircleView;

#[view]
pub struct Slider {
    circle:    Sub<CircleView>,
    raw_value: f32,

    converter: Converter,

    pub on_change: Event<f32>,
}

impl Slider {
    pub fn value(&self) -> f32 {
        self.converter.convert(self.raw_value)
    }

    pub fn set_value(&mut self, val: impl IntoF32) -> &mut Self {
        self.raw_value = self.converter.reverse_convert(val);

        let val = 1.0 - self.raw_value;

        let circle_range = self.height() - self.circle.frame().height();
        let y_pos = circle_range * val;
        self.circle.set_y(y_pos);

        self.value_changed();
        self
    }

    pub fn indicator_position(&self) -> f32 {
        self.circle.frame().center().y
    }

    pub fn set_range(&mut self, min: impl IntoF32, max: impl IntoF32) -> &mut Self {
        self.set_min(min).set_max(max);
        self.value_changed();
        self
    }

    pub fn set_min(&mut self, min: impl IntoF32) -> &mut Self {
        self.converter.set_min(min);
        self.value_changed();
        self
    }

    pub fn set_max(&mut self, max: impl IntoF32) -> &mut Self {
        self.converter.set_max(max);
        self.value_changed();
        self
    }

    fn value_changed(&self) {
        self.on_change.trigger(self.value());
    }
}

impl ViewSetup for Slider {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.touch.all.val(move |touch| {
            self.on_touch(&touch);
        });

        self.circle.set_color(Color::BLUE);
    }
}

impl ViewCallbacks for Slider {
    fn update(&mut self) {
        let radius = self.width() / 2.0;
        self.circle.set_radius(radius);
    }
}

impl Slider {
    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            return;
        }

        let half_circle = self.circle.frame().height() / 2.0;
        let y_pos = touch.position.y.clamp(half_circle, self.height() - half_circle);

        self.circle.set_y(y_pos - half_circle);
        self.raw_value = 1.0 - (y_pos - half_circle) / (self.height() - half_circle * 2.0);

        self.value_changed();
    }
}
