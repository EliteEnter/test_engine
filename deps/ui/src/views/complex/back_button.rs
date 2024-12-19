use refs::Weak;
use ui_proc::view;

use crate::{
    Button, Setup,
    has_data::HasText,
    view::{ViewController, ViewData},
};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct BackButton {
    #[init]
    button: Button,
}

impl Setup for BackButton {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Back");
        self.button.place().back();

        self.button.on_tap(move || {
            self.navigation().pop();
        });
    }
}
