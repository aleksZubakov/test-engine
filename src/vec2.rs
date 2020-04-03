use std::ops;

#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}


impl Vec2 {
    pub fn create(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn clone(&self) -> Self {
        Vec2 { x: self.x, y: self.y }
    }
    fn sum(&self, other: &Vec2) -> Vec2 {
        Vec2::create(self.x + other.x, self.y + other.y)
    }
    fn sub_func(&self, other: &Vec2) -> Vec2 {
        Vec2::create(self.x - other.x, self.y - other.y)
    }

    fn el_product(&self, other: &Vec2) -> Vec2 {
        Vec2::create(self.x * other.x, self.y * other.y)
    }

    fn scalar_product(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f64 {
        self.scalar_product(self).sqrt()
    }
    pub fn magnitude_sqr(&self) -> f64 {
        self.scalar_product(self)
    }
}

impl ops::Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        self.sum(&other)
    }
}

impl ops::Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, other: &Vec2) -> Vec2 {
        self.sub_func(&other)
    }
}

impl ops::AddAssign<&Vec2> for Vec2 {
    fn add_assign(&mut self, other: &Self) {
        *self = self.sum(other)
    }
}

impl ops::SubAssign<&Vec2> for Vec2 {
    fn sub_assign(&mut self, other: &Self) {
        *self = self.sub_func(other)
    }
}

impl ops::BitXor<&Vec2> for &Vec2 {
    type Output = f64;

    fn bitxor(self, other: &Vec2) -> f64 {
        self.scalar_product(other)
    }
}

impl ops::Mul<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn mul(self, other: &Vec2) -> Vec2 {
        self.el_product(other)
    }
}
