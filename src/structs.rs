use std::ops::{Div, Mul};

use num_traits::AsPrimitive;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Dimensions<T> {
    pub width: T,
    pub height: T
}

impl<T> Dimensions<T> {
    #[inline]
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T, U: Into<T>> From<(U, U)> for Dimensions<T> {
    #[inline]
    fn from((w, h): (U, U)) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl<T, U: Into<T>> From<[U; 2]> for Dimensions<T> {
    #[inline]
    fn from([w, h]: [U; 2]) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl<T: AsPrimitive<f32>> Div<f32> for Dimensions<T> where f32: AsPrimitive<T> {
    type Output = Self;

    fn div(self, with: f32) -> Self::Output {
        let [width, height] = [self.width, self.height]
        .map(|size| (size.as_() / with).round().as_());

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
