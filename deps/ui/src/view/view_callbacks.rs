use crate::{Touch, View};

pub trait ViewCallbacks {
    fn setup(&mut self);
    fn update(&mut self);
    fn on_touch(&mut self, touch: &Touch);
    fn on_selection_changed(&mut self, selected: bool);
}

impl<T: ?Sized + View> ViewCallbacks for T {
    default fn setup(&mut self) {}
    default fn update(&mut self) {}
    default fn on_touch(&mut self, _: &Touch) {}
    default fn on_selection_changed(&mut self, _: bool) {}
}
