
use crate::gm::Rect;
use crate::utils::ArrayView;
use crate::gl_wrapper::{Buffer, BufferConfig};

const RECT: Rect = Rect::make(-1.0, -1.0, 2.0, 2.0);

const RECT_INDICES: [u32; 4] = [0, 1, 3, 4 ];
const INDICES:      [u32; 4] = [0, 1, 2, 3 ];

const FULLSCREEN_VERT: [f32; 8] = [
    RECT.origin.x,                   RECT.origin.y,
    RECT.origin.x,                   RECT.size.height + RECT.origin.y,
    RECT.size.width + RECT.origin.x, RECT.size.height + RECT.origin.y,
    RECT.size.width + RECT.origin.x, RECT.origin.y
];

const IMAGE_VERTICES: [f32; 16] = [
    RECT.origin.x,                   RECT.origin.y,                    0.0,  1.0, //|- |
    RECT.origin.x,                   RECT.size.height + RECT.origin.y, 0.0,  0.0, //|_ |
    RECT.size.width + RECT.origin.x, RECT.size.height + RECT.origin.y, 1.0,  0.0, //| _|
    RECT.size.width + RECT.origin.x, RECT.origin.y,                    1.0,  1.0  //| -|
];

const OUTLINE_VERTICES: [f32; 8] = [
    RECT.origin.x,                   RECT.origin.y,
    RECT.origin.x,                   RECT.size.height + RECT.origin.y,
    RECT.size.width + RECT.origin.x, RECT.size.height + RECT.origin.y,
    RECT.size.width + RECT.origin.x, RECT.origin.y
];

pub struct Buffers {
    pub fullscreen:         Buffer,
    pub fullscreen_image:   Buffer,
    pub fullscreen_outline: Buffer
}

impl Buffers {
    pub fn init() -> Buffers {

        let fullscreen = Buffer::new(
            &BufferConfig::_2,
            ArrayView::from_data(&FULLSCREEN_VERT[0], FULLSCREEN_VERT.len()),
            Some(ArrayView::from_data(&RECT_INDICES[0], RECT_INDICES.len())),
            gl::TRIANGLE_STRIP
        );

        let fullscreen_image = Buffer::new(
            &BufferConfig::_2_2,
            ArrayView::from_data(&IMAGE_VERTICES[0], IMAGE_VERTICES.len()),
            Some(ArrayView::from_data(&RECT_INDICES[0], RECT_INDICES.len())),
            gl::TRIANGLE_STRIP
        );

        let fullscreen_outline = Buffer::new(
            &BufferConfig::_2,
            ArrayView::from_data(&OUTLINE_VERTICES[0], OUTLINE_VERTICES.len()),
            Some(ArrayView::from_data(&INDICES[0], INDICES.len())),
            gl::LINE_LOOP
        );

        Buffers {
            fullscreen,
            fullscreen_image,
            fullscreen_outline
        }
    }
}