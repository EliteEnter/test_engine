use std::{
    borrow::Borrow,
    ops::{Div, Mul},
};

use rtools::IntoF32;
use serde::{Deserialize, Serialize};

use crate::flat::{Point, Rect};

#[derive(Copy, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeBase<T> {
    pub width:  T,
    pub height: T,
}

pub type Size = SizeBase<f32>;

impl Size {
    pub fn diagonal(&self) -> f32 {
        (self.width * self.width + self.height * self.height).sqrt()
    }

    pub fn square(side: f32) -> Size {
        (side, side).into()
    }

    pub fn is_valid(&self) -> bool {
        self.width > 0.0 || self.height > 0.0
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    pub fn is_negative(&self) -> bool {
        self.width < 0.0 || self.height < 0.0
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.width / 2.0,
            y: self.height / 2.0,
        }
    }

    pub fn fit_height(&self, height: impl IntoF32) -> Size {
        let ratio = height.into_f32() / self.height;
        *self * ratio
    }

    pub fn fit_in(&self, rect: impl Borrow<Rect>) -> Rect {
        let rect = rect.borrow();
        let ratio = rect.size.height / self.height;
        let size = *self * ratio;
        let x = rect.x() + rect.width() / 2.0 - size.width / 2.0;
        (x, rect.y(), size.width, size.height).into()
    }
}

impl<W: IntoF32, H: IntoF32> From<(W, H)> for Size {
    fn from(tup: (W, H)) -> Self {
        Self {
            width:  tup.0.into_f32(),
            height: tup.1.into_f32(),
        }
    }
}

impl<T: IntoF32> Mul<T> for Size {
    type Output = Size;
    fn mul(self, rhs: T) -> Self::Output {
        (self.width * rhs.into_f32(), self.height * rhs.into_f32()).into()
    }
}

impl<T: IntoF32> Div<T> for Size {
    type Output = Size;
    fn div(self, rhs: T) -> Self::Output {
        (self.width / rhs.into_f32(), self.height / rhs.into_f32()).into()
    }
}

impl ToString for Size {
    fn to_string(&self) -> String {
        format!("width: {:.2}, height: {:.2}", self.width, self.height)
    }
}