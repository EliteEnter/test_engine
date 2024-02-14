use anyhow::Result;
use gm::{
    flat::{Rect, Size},
    Color,
};
use wgpu::{Device, Queue, RenderPass, TextureFormat};

use crate::{
    image::Image,
    render::{image_state::ImageState, rect_state::RectState},
};

#[derive(Debug)]
pub struct WGPUDrawer {
    pub window_size:     Size,
    pub device:          Device,
    pub queue:           Queue,
    rect_state:          RectState,
    colored_image_state: ImageState,
}

impl WGPUDrawer {
    pub fn new(device: Device, queue: Queue, texture_format: TextureFormat) -> Result<Self> {
        let rect_state = RectState::new(&device, texture_format);
        let colored_image_state = ImageState::new(&device);
        Ok(Self {
            window_size: Default::default(),
            device,
            queue,
            rect_state,
            colored_image_state,
        })
    }
}

impl WGPUDrawer {
    pub fn fill_rect<'a>(&'a self, render_pass: &mut RenderPass<'a>, rect: &Rect, color: &Color) {
        self.rect_state.draw(&self.device, render_pass, rect, color);
    }

    pub fn draw_image<'a>(&'a self, render_pass: &mut RenderPass<'a>, image: &'static Image, rect: &Rect) {
        self.colored_image_state.draw(image, rect, render_pass);
    }
}
