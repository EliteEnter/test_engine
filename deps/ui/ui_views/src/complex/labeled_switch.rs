use refs::Weak;
use ui::{layout::Anchor, view, Event, SubView, ToLabel, ViewSetup};

use crate::{Label, Switch};

#[view]
pub struct LabeledSwitch {
    label:  SubView<Label>,
    switch: SubView<Switch>,

    pub selected: Event<bool>,
}

impl LabeledSwitch {
    pub fn text(&self) -> &str {
        self.label.text()
    }

    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.label.set_text(text);
        self
    }
}

impl ViewSetup for LabeledSwitch {
    fn setup(self: Weak<Self>) {
        self.label.place.blt(0).relative(Anchor::Width, 0.5, self);
        self.switch
            .place
            .size(80, 40)
            .center_ver()
            .between_super(self.label, Anchor::Right);

        self.switch.selected.val(move |on| self.selected.trigger(on));
    }
}
