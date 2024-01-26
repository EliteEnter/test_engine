use std::sync::OnceLock;

use bytemuck::checked::cast_slice;
#[cfg(mobile)]
use gles31_sys::*;
use gm::flat::{Point, Rect};

use crate::{Buffer, BufferConfig};

const RECT: Rect = Rect::from_vals([-1.0, -1.0, 2.0, 2.0]);

pub const RECT_INDICES: &[u16; 4] = &[0, 1, 3, 2];
const INDICES: &[u16; 4] = &[0, 1, 2, 3];

pub const FULLSCREEN_VERT: &[Point] = &[
    Point::new(RECT.origin.x, RECT.origin.y),
    Point::new(RECT.origin.x, RECT.size.height + RECT.origin.y),
    Point::new(RECT.size.width + RECT.origin.x, RECT.size.height + RECT.origin.y),
    Point::new(RECT.size.width + RECT.origin.x, RECT.origin.y),
];

const IMAGE_VERTICES: &[f32; 16] = &[
    RECT.origin.x,
    RECT.origin.y,
    0.0,
    1.0, //|- |
    RECT.origin.x,
    RECT.size.height + RECT.origin.y,
    0.0,
    0.0, //|_ |
    RECT.size.width + RECT.origin.x,
    RECT.size.height + RECT.origin.y,
    1.0,
    0.0, //| _|
    RECT.size.width + RECT.origin.x,
    RECT.origin.y,
    1.0,
    1.0, //| -|
];

static BUFFERS: OnceLock<Buffers> = OnceLock::new();

pub struct Buffers {
    pub full:         Buffer,
    pub full_image:   Buffer,
    pub full_outline: Buffer,
}

impl Buffers {
    fn init() -> Buffers {
        trace!("Initializing buffers");

        let full = Buffer::make(
            &BufferConfig::_2,
            cast_slice(FULLSCREEN_VERT),
            Some(RECT_INDICES),
            GLC!(TRIANGLE_STRIP),
        );

        let full_image = Buffer::make(
            &BufferConfig::_2_2,
            IMAGE_VERTICES,
            Some(RECT_INDICES),
            GLC!(TRIANGLE_STRIP),
        );

        let full_outline = Buffer::make(
            &BufferConfig::_2,
            cast_slice(FULLSCREEN_VERT),
            Some(INDICES),
            GLC!(LINE_LOOP),
        );

        trace!("Buffers: OK");

        Buffers {
            full,
            full_image,
            full_outline,
        }
    }

    pub fn get() -> &'static Buffers {
        BUFFERS.get_or_init(Self::init)
    }
}
