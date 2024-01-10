use std::ops;

#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
