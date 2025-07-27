use std::ops::{Div, Mul};

use num_traits::AsPrimitive;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32
}

impl Dimensions {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl<T: Into<u32>> From<(T, T)> for Dimensions {
    #[inline]
    fn from((w, h): (T, T)) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl<T: Into<u32>> From<[T; 2]> for Dimensions {
    #[inline]
    fn from([w, h]: [T; 2]) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl Div<f32> for Dimensions {
    type Output = Self;

    fn div(self, with: f32) -> Self::Output {
        let [width, height] = [self.width, self.height]
        .map(|size| (size as f32 / with).round() as u32);

        Self::new(width, height)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position<T> {
    pub x: T,
    pub y: T
}

impl<T> Position<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T, U: Into<T>> From<(U, U)> for Position<T> {
    #[inline]
    fn from((x, y): (U, U)) -> Self {
        Self::new(x.into(), y.into())
    }
}

impl<T, U: Into<T>> From<[U; 2]> for Position<T> {
    #[inline]
    fn from([x, y]: [U; 2]) -> Self {
        Self::new(x.into(), y.into())
    }
}

impl<T: AsPrimitive<f32>> Mul<f32> for Position<T>
    where f32: AsPrimitive<T>
{
    type Output = Self;

    fn mul(self, with: f32) -> Self::Output {
        let [x, y] = [self.x, self.y]
        .map(|coord| (coord.as_() * with).round().as_());

        Self::new(x, y)
    }
}

impl<T: AsPrimitive<f32>> Div<f32> for Position<T>
    where f32: AsPrimitive<T>
{
    type Output = Self;

    fn div(self, with: f32) -> Self::Output {
        let [x, y] = [self.x, self.y]
        .map(|coord| (coord.as_() / with).round().as_());

        Self::new(x, y)
    }
}
