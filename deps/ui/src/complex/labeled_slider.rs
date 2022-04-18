use rtools::{Event, IntoF32, Rglica};

use crate::{
    complex::Slider,
    view_base::{add_view_with_frame, ViewBase},
    Label, View,
};

#[derive(Default, Debug)]
pub struct LabeledSlider {
    base:   ViewBase,
    label:  Rglica<Label>,
    slider: Rglica<Slider>,

    pub on_change: Event<f32>,
}

impl LabeledSlider {
    pub fn set_start_value(&mut self, value: impl IntoF32) {
        self.slider.start_value = value.into_f32()
    }

    pub fn set_multiplier(&mut self, multiplier: impl IntoF32) {
        self.slider.multiplier = multiplier.into_f32()
    }

    fn on_change(&mut self, val: f32) {
        self.label.set_text(val);
        self.on_change.trigger(val);
    }
}

impl View for LabeledSlider {
    fn setup(&mut self) {
        let frames = self.place().frames_for_ratio([1, 5]);

        self.label = add_view_with_frame(self, frames[0]);
        self.slider = add_view_with_frame(self, frames[1]);

        self.slider.on_change.set(self, move |a, this| this.on_change(a));
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
