use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use bytemuck::{Pod, Zeroable};
use rtools::IntoF32;
use serde::{Deserialize, Serialize};

use crate::flat::Size;

#[derive(Copy, Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointBase<T> {
    pub x: T,
    pub y: T,
}

impl<T> PointBase<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub type Point = PointBase<f32>;

unsafe impl Zeroable for Point {}
unsafe impl Pod for Point {}

impl Point {
    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn angle_to(&self, point: Point) -> f32 {
        let target = point - *self;
        target.angle()
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn middle(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    pub fn positive(&self) -> bool {
        self.x >= 0.0 && self.y >= 0.0
    }
}

impl Point {
    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn invert_x(&mut self) {
        self.x = -self.x
    }

    pub fn invert_y(&mut self) {
        self.y = -self.y
    }
}

impl Point {
    pub fn normalized(self) -> Point {
        self.with_length(1.0)
    }

    pub fn normalize(&mut self) {
        self.set_length(1.0)
    }
}

impl Point {
    pub fn with_length(self, l: f32) -> Point {
        let ratio = l / self.length();
        Point {
            x: self.x * ratio,
            y: self.y * ratio,
        }
    }

    pub fn set_length(&mut self, l: f32) {
        let ratio = l / self.length();
        self.x *= ratio;
        self.y *= ratio;
    }

    pub fn trim(&mut self, max_length: f32) {
        if self.length() < max_length {
            return;
        }
        self.set_length(max_length)
    }

    pub fn trimmed(mut self, max_length: f32) -> Point {
        self.trim(max_length);
        self
    }
}

impl PointBase<i32> {
    pub fn is_negative(&self) -> bool {
        self.x < 0 || self.y < 0
    }
}

impl<T: Add<Output = T>> Add for PointBase<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> Add for &PointBase<T> {
    type Output = PointBase<T>;
    fn add(self, rhs: &Self::Output) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

impl<T: Sub<Output = T>> Sub for PointBase<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &PointBase<T> {
    type Output = PointBase<T>;
    fn sub(self, rhs: &Self::Output) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y
    }
}

impl<T: IntoF32> Mul<T> for Point {
    type Output = Point;
    fn mul(self, rhs: T) -> Point {
        let rhs = rhs.into_f32();
        (self.x * rhs, self.y * rhs).into()
    }
}

impl Mul<Size> for Point {
    type Output = Point;
    fn mul(self, size: Size) -> Point {
        (self.x * size.width, self.y * size.height).into()
    }
}

impl<T: IntoF32> MulAssign<T> for Point {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into_f32();
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: IntoF32> Div<T> for Point {
    type Output = Point;
    fn div(self, rhs: T) -> Point {
        let rhs = rhs.into_f32();
        (self.x / rhs, self.y / rhs).into()
    }
}

impl<T: IntoF32> DivAssign<T> for Point {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into_f32();
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<X, Y> const From<(X, Y)> for Point
where
    X: ~const IntoF32,
    Y: ~const IntoF32,
{
    fn from(tup: (X, Y)) -> Self {
        Self {
            x: tup.0.into_f32(),
            y: tup.1.into_f32(),
        }
    }
}

impl<T: IntoF32> Display for PointBase<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {:.2}, y: {:.2}", self.x.into_f32(), self.y.into_f32())
    }
}
