use std::ops::Div;


#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn from((w, h): (T, T)) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl<T: Into<u32>> From<[T; 2]> for Dimensions {
    fn from([w, h]: [T; 2]) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl Div<f32> for Dimensions {
    type Output = Self;

    #[inline]
    fn div(self, with: f32) -> Self::Output {
        let [width, height] = [self.width, self.height]
        .map(|d| (d as f32 / with).round() as u32);
    
        Self::new(width, height)
    }
}