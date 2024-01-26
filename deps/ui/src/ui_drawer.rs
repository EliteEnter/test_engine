use std::{fmt::Debug, ops::DerefMut};

use gl_image::Image;
use gl_wrapper::path_data::{DrawMode, PathData};
use gm::{flat::Rect, Color};
use refs::Weak;

use crate::{
    view::{ViewAnimation, ViewSubviews},
    View, ViewLayout,
};

pub trait UIDrawer: Debug + Send {
    fn fill(&self, rect: &Rect, color: &Color, priority: usize);
    fn outline(&self, rect: &Rect, color: &Color, priority: usize);
    fn draw_image(&self, image: &Image, rect: &Rect, color: &Color, priority: usize, is_text: bool);
    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>, priority: usize);
    fn draw(&self, view: &dyn View);
    fn set_root_frame(&mut self, frame: Rect);

    fn update_internal(&self, view: &mut dyn View) {
        if view.is_hidden {
            return;
        }
        view.layout();
        view.commit_animations();
        view.calculate_absolute_frame();
        view.update();
        for view in view.subviews_mut() {
            self.update_internal((*view).deref_mut());
        }
    }

    fn update(&self, view: &mut Weak<dyn View>) {
        self.update_internal(view.deref_mut());
        self.draw(view.deref_mut());
    }
}
