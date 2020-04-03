use super::Vec2;

pub type Point2D = Vec2;

pub struct Line2D {
    pub start_point: Vec2,
    pub end_point: Vec2,
}

impl Line2D {
    pub fn create(start_point: Vec2, end_point: Vec2) -> Line2D {
        Line2D { start_point, end_point }
    }

    pub fn len(&self) -> f64 {
        (&self.start_point - &self.end_point).magnitude()
    }

    pub fn len_sqr(&self) -> f64 {
        (&self.start_point - &self.end_point).magnitude_sqr()
    }
}

pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

impl Circle {
    pub fn create(center: Vec2, radius: f64) -> Circle {
        Circle { center, radius }
    }
}

pub struct Rectangle2D {
    pub start_point: Vec2,
    pub size: Vec2,
}

impl Rectangle2D {
    pub fn create(start_point: Vec2, size: Vec2) -> Rectangle2D {
        Rectangle2D { start_point, size }
    }

    pub fn from_min_max(min: Vec2, max: Vec2) -> Rectangle2D {
        Rectangle2D {
            size: &max + &min,
            start_point: min,
        }
    }

    pub fn min_point(&self) -> Vec2 {
        self.start_point.clone()
    }

    pub fn max_point(&self) -> Vec2 {
        &self.start_point + &self.size
    }

    pub fn min_max_points(&self) -> (Vec2, Vec2) {
        (self.max_point(), self.max_point())
    }
}


