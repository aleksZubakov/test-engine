use std::ops;

#[derive(Debug)]
pub struct Vec2(pub f64, pub f64);


impl Vec2 {
    pub fn create(x: f64, y: f64) -> Vec2 {
        Vec2(x, y)
    }

    pub fn clone(&self) -> Self {
        let Vec2(x, y) = *self;
        Vec2(x, y)
    }
    fn sum(&self, other: &Vec2) -> Vec2 {
        Vec2::create(self.0 + other.0, self.1 + other.1)
    }
    fn sub_func(&self, other: &Vec2) -> Vec2 {
        Vec2::create(self.0 - other.0, self.1 - other.1)
    }

    fn el_product(&self, other: &Vec2) -> Vec2 {
        Vec2::create(self.0 * other.0, self.1 * other.1)
    }

    fn scalar_product(&self, other: &Vec2) -> f64 {
        self.0 * other.0 + self.1 * other.1
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
