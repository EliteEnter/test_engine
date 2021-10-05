use gl_wrapper::{Buffer, BufferConfig};
use gm::{flat::PointsPath, Color};
use proc_macro::Boxed;

use crate::{
    complex::{path_data::DrawMode, PathData},
    View, ViewBase,
};

#[derive(Boxed)]
pub struct DrawingView {
    base:      ViewBase,
    pub paths: Vec<PathData>,
}

impl DrawingView {
    pub fn add_path(&mut self, path: PointsPath, color: Color) {
        self.paths
            .push(initialize_path_data(path, color, DrawMode::Fill))
    }
}

impl View for DrawingView {
    fn paths(&self) -> Option<&[PathData]> { Some(&self.paths) }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}

fn initialize_path_data(path: PointsPath, color: Color, draw_mode: DrawMode) -> PathData {
    // #[cfg(any(target_os = "ios", target_os = "android"))]
    // use gles31_sys::GL_LINE_STRIP;

    let buffer = Buffer::make(
        &BufferConfig::_2,
        (&path.points).into(),
        None,
        6, //GLC!(GL_TRIANGLE_FAN), //draw_mode.to_gl(),
    );

    PathData {
        buffer,
        path,
        color,
        draw_mode,
    }
}
