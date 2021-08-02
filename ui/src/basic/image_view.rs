use crate::{View, ViewBase};
use gl_image::Image;
use proc_macro::AsAny;
use tools::refs::MutWeak;
use tools::weak_self::HasWeakSelf;
use tools::New;

#[derive(Debug, AsAny)]
pub struct ImageView {
    pub image: Image,
    base: ViewBase,
    _weak: MutWeak<ImageView>,
}

impl New for ImageView {
    fn new() -> Self {
        Self {
            image: Image::new(),
            base: ViewBase::new(),
            _weak: MutWeak::new(),
        }
    }
}

impl HasWeakSelf for ImageView {
    fn weak(&self) -> MutWeak<Self> {
        self._weak.clone()
    }

    fn set_weak(&mut self, weak: MutWeak<Self>) {
        self._weak = weak
    }
}

impl View for ImageView {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }

    fn image(&self) -> Option<Image> {
        Some(self.image)
    }
}