use crate::gm::{Color, Rect};
use crate::ui::input::Touch;
use crate::ui::view::View;
use crate::ui::{Font, ImageView, ViewBase};
use std::any::Any;
use std::ops::{Deref, DerefMut};
use tools::refs::{make_shared, DynWeak, MutWeak, Shared};
use tools::weak_self::HasWeakSelf;
use tools::{AsAny, New};

pub struct Label {
    pub font: Font,
    base: ViewBase,
    _weak: MutWeak<Label>,
}

impl Label {
    pub fn set_text(&mut self, text: &str) {
        self.remove_all_subviews();

        if text.is_empty() {
            return;
        }

        let mut last_max_x: f32 = 0.0;
        let mut advance: f32 = 0.0;
        let mut content_size = self.base.frame().size;

        content_size.height = self.font.height;

        for letter in text.chars() {
            let glyph = self.font.glyph_for_char(letter);

            let mut glyph_view = ImageView::new();

            glyph_view.set_frame(Rect::from_size(&glyph.size()));
            glyph_view.image = glyph.image;

            glyph_view.set_frame(Rect::make(
                advance + glyph.bearing.x,
                content_size.height - glyph.bearing.y + self.font.baseline_shift,
                glyph.size().width,
                glyph.size().height,
            ));

            last_max_x = glyph_view.frame().max_x();

            advance += glyph.advance as f32;

            self.add_subview(make_shared(glyph_view));
        }

        content_size.width = last_max_x;

        let rect = Rect::make(
            self.view().frame().origin.x,
            self.view().frame().origin.y,
            content_size.width,
            content_size.height,
        );

        self.set_frame(rect);
    }
}

impl AsAny for Label {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl New for Label {
    fn new() -> Self {
        Self {
            font: Font::blank(),
            base: ViewBase::new(),
            _weak: MutWeak::new(),
        }
    }
}

impl HasWeakSelf for Label {
    fn weak(&self) -> MutWeak<Self> {
        self._weak.clone()
    }

    fn set_weak(&mut self, weak: MutWeak<Self>) {
        self._weak = weak
    }
}

impl Deref for Label {
    type Target = ViewBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Label {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl View for Label {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
