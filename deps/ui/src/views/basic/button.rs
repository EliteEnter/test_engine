use gm::{Color, ToF32};
use refs::Weak;
use ui_proc::view;
use vents::Event;
use wgpu_wrapper::image::ToImage;

use crate::{
    view::{ViewData, ViewTouch},
    ImageView, Label, Sub, ToLabel, ViewSetup,
};

mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct Button {
    label: Sub<Label>,
    image: Sub<ImageView>,

    on_tap: Event,
}

impl Button {
    pub fn on_tap(&self, action: impl FnMut() + 'static) -> &Self {
        self.enable_touch();
        self.on_tap.sub(action);
        self
    }

    pub fn text(&self) -> &str {
        &self.label.text
    }

    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.label.set_hidden(false);
        self.label.text = text.to_label();
        self
    }

    pub fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.image.set_hidden(false);
        self.image.set_image(image);
        self
    }

    pub fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.label.set_text_color(color);
        self
    }

    pub fn set_text_size(&mut self, size: impl ToF32) -> &mut Self {
        self.label.set_text_size(size);
        self
    }
}

impl ViewSetup for Button {
    fn setup(mut self: Weak<Self>) {
        self.label.place().back();
        self.label.set_hidden(true);

        self.image.place().back();
        self.image.set_hidden(true);

        self.touch.up_inside.sub(move || self.on_tap.trigger(()));
    }
}

#[macro_export]
macro_rules! link_button {
    ($self:ident, $($button:ident).+, $method:ident) => {{
        use test_engine::ui::AlertErr;
        $self.$($button).+.on_tap(move || { $self.$method().alert_err(); });
    }}
}

#[macro_export]
macro_rules! async_link_button {
    ($self:ident, $($button:ident).+, $method:ident) => {
        $self.$($button).+.on_tap(move || {
            tokio::spawn(async move {
                use test_engine::ui::AlertErr;
                $self.$method().await.alert_err();
            });
        });
    };
}

#[macro_export]
macro_rules! async_call {
    ($self:ident, $method:ident) => {
        tokio::spawn(async move {
            $self.$method().await.alert_err();
        });
    };
}