use super::Vec2;

#[derive(Debug, Default)]
pub struct ElemSize(Vec2);

impl ElemSize {
    pub fn new(width: f64, height: f64) -> Self { Self(Vec2::new(width, height)) }

    pub fn width(&self) -> f64 { self.0.x }
    pub fn height(&self) -> f64 { self.0.y }

    pub fn as_vec2(&self) -> &Vec2 { &self.0 }
}
