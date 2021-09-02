use crate::basic::Placer;
use crate::{make_view_on, Touch, View, ViewBase};
use gl_image::Image;
use gm::{Color, Rect};
use proc_macro::{AsAny, New};
use std::ops::DerefMut;
use tools::{Event, Rglica};

#[derive(AsAny, New)]
pub struct SubviewsTestView {
    base: ViewBase,
    first: Rglica<ViewBase>,
    second: Rglica<ViewBase>,
    third: Rglica<ViewBase>,
    forth: Rglica<ViewBase>,
    fifth: Rglica<ViewBase>,
}

impl View for SubviewsTestView {
    fn setup(&mut self) {
        self.frame_mut().size = (120, 120).into();

        self.first = make_view_on(self);
        // self.second = make_view_on(self.first.deref_mut());
        // self.third = make_view_on(self.second.deref_mut());
        // self.forth = make_view_on(self.third.deref_mut());
        // self.fifth = make_view_on(self.forth.deref_mut());

        self.first.frame_mut().size = (100, 100).into();
        // self.second.frame_mut().size = (90, 90).into();
        // self.third.frame_mut().size = (80, 80).into();
        // self.forth.frame_mut().size = (70, 70).into();
        // self.fifth.frame_mut().size = (60, 60).into();

        // self.second.placer().at_center();
        // self.third.placer().at_center();
        // self.forth.placer().at_center();
        // self.fifth.placer().at_center();
    }

    fn layout(&mut self) {
        self.first.placer().at_center();
        dbg!(self.first.placer());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}