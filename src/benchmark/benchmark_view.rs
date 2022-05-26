use std::borrow::Borrow;

use test_engine::{
    main_view::{HasLevel, MainView},
    rtools::{Rglica, ToRglica},
    sprites::Player,
    ui::{
        basic::Button, impl_view, placer::Anchor, view, Label, View, ViewBase, ViewCallbacks, ViewFrame,
        ViewSubviews,
    },
    ui_layer::UILayer,
    Level,
};

use crate::{benchmark::benchmark_level::BenchmarkLevel, TestGameView};

#[view]
#[derive(Debug, Default)]
pub struct BenchmarkView {
    level:         BenchmarkLevel,
    bullets_label: Rglica<Label>,

    back: Rglica<Button>,

    ui: Rglica<UILayer>,
}

impl_view!(BenchmarkView);

impl ViewCallbacks for BenchmarkView {
    fn setup(&mut self) {
        self.level.setup();

        self.back = self.add_view();
        self.back.set_text("Back").set_frame((120, 20));
        self.back.on_tap.set(self, |this, _| {
            this.ui.set_view::<TestGameView>();
        });

        self.bullets_label = self.add_view();
        self.bullets_label.set_frame((120, 20));
    }

    fn layout(&mut self) {
        self.back.place().bottom_center(20);
        self.bullets_label
            .place()
            .anchor(self.back, Anchor::Top, Anchor::Center, 10);
    }

    fn update(&mut self) {
        self.bullets_label
            .set_text(format!("Bullets: {}", self.level.bullets_count));
    }
}

impl MainView for BenchmarkView {
    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}

impl HasLevel for BenchmarkView {
    fn player(&self) -> Rglica<Player> {
        self.level.player
    }

    fn level(&self) -> Rglica<dyn Level> {
        (self.level.borrow() as &dyn Level).to_rglica()
    }
}