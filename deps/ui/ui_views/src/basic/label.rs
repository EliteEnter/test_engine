use gm::Color;
use refs::Weak;
use rtools::{data_manager::Handle, IntoF32};
use text::{render_text, Font};
use ui::{view, SubView, ViewCallbacks, ViewData, ViewFrame, ViewSetup};

use crate::ImageView;

#[view]
pub struct Label {
    font:          Handle<Font>,
    text:          String,
    prev_text:     String,
    image_view:    SubView<ImageView>,
    text_size:     f32,
    needs_update:  bool,
    pub free_text: bool,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        let text = text.to_string();
        if text.is_empty() {
            self.image_view.set_hidden(true);
            return self;
        }

        self.image_view.set_hidden(false);

        if self.text == text {
            return self;
        }
        self.text = text;
        self.needs_update = true;
        self
    }

    pub fn set_text_size(&mut self, size: impl IntoF32) -> &mut Self {
        self.text_size = size.into_f32();
        self
    }

    pub fn append_text(&mut self, text: impl ToString) -> &mut Self {
        self.set_text(format!("{}{}", self.text, text.to_string()));
        self
    }

    pub fn pop_letter(&mut self) {
        if !self.text.is_empty() {
            self.text.pop();
            self.needs_update = true;
        }
    }

    pub fn set_text_color(&mut self, _color: impl Into<Color>) -> &mut Self {
        //self.image_view.view_mut().image.color = color.into();
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.set_text("")
    }

    fn fit_size(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let image = self.image_view.image();

        let size = if image.size.width > self.width() {
            image.size.fit_width(self.width())
        } else if image.size.height > self.height() {
            image.size.fit_height(self.height())
        } else {
            image.size
        };

        self.image_view.set_size(size);
    }

    fn set_letters(&mut self) {
        if self.free_text {
            for char in self.prev_text.chars() {
                if char.is_ascii_digit() {
                    self.image_view.image().free();
                    break;
                }
            }
        }

        let image = render_text(&self.text, &self.font, self.text_size);
        self.image_view.set_image(image);
        self.prev_text = self.text.clone();
    }
}

impl ViewSetup for Label {
    fn setup(mut self: Weak<Self>) {
        self.font = Font::san_francisco();
        self.set_size((100, 20));
        self.text_size = 32.0;

        debug_assert!(self.text.is_empty());
        self.image_view.place.center();
        self.image_view.set_hidden(true);
    }
}

impl ViewCallbacks for Label {
    fn update(&mut self) {
        if self.needs_update {
            self.set_letters();
            self.needs_update = false;
        }
        self.fit_size();
    }
}
