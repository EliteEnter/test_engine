use std::hash::{Hash, Hasher};

use bytemuck::{Pod, Zeroable};
use rtools::Random;

use crate::{color::ColorBase, U8Color};

pub type Color = ColorBase<f32>;
unsafe impl Zeroable for Color {}
unsafe impl Pod for Color {}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub fn is_clear(&self) -> bool {
        !self.is_visible()
    }

    pub fn is_visible(&self) -> bool {
        self.a > 0.02
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn to_u8(&self) -> U8Color {
        U8Color::rgba(
            (255.0 * self.r) as u8,
            (255.0 * self.g) as u8,
            (255.0 * self.b) as u8,
            (255.0 * self.a) as u8,
        )
    }
}

impl Color {
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 0.8);
    pub const LIGHT_BLUE: Color = Color::rgb(0.0, 0.7, 1.0);
    pub const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
    pub const ORANGE: Color = Color::rgb(1.0, 0.6, 0.0);
    pub const PURPLE: Color = Color::rgb(1.0, 0.0, 1.0);
    pub const TURQUOISE: Color = Color::rgb(0.0, 1.0, 1.0);
    pub const GRAY: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const BROWN: Color = Color::rgb(0.7, 0.4, 0.2);
    pub const LIGHT_GRAY: Color = Color::rgb(0.8, 0.8, 0.8);
    pub const LIGHTER_GRAY: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const CLEAR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

    pub const ALL: [Color; 12] = [
        Color::BLACK,
        Color::WHITE,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::LIGHT_BLUE,
        Color::YELLOW,
        Color::ORANGE,
        Color::PURPLE,
        Color::TURQUOISE,
        Color::BROWN,
        Color::LIGHT_GRAY,
    ];

    pub fn random() -> Color {
        Color::ALL[usize::random_in(0..Color::ALL.len())]
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.r.to_bits());
        state.write_u32(self.g.to_bits());
        state.write_u32(self.b.to_bits());
        state.write_u32(self.a.to_bits());
        state.finish();
    }
}

#[test]
fn color_diff() {
    assert_eq!(Color::WHITE.diff(Color::CLEAR), 4.0);
    assert_eq!(Color::WHITE.diff(Color::WHITE), 0.0);
    assert_eq!(Color::WHITE.diff(Color::WHITE.with_alpha(0.9)), 0.100000024);
}

#[test]
fn color_to_u8() {
    assert_eq!(
        Color::rgba(0.5, 1.0, 0.1, 0.0).to_u8(),
        U8Color::rgba(127, 255, 25, 0)
    );
}