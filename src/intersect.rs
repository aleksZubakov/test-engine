use std::prelude::v1::Vec;

use crate::primitives::{Circle, Line2D, Point2D, Rectangle2D};
use crate::vec2::Vec2;

const EPS: f64 = 0.0001_f64;

pub fn is_point_on_line(point: &Vec2, line: &Line2D) -> bool {
    let Vec2(x1, y1) = line.start_point;
    let Vec2(x2, y2) = line.end_point;

    let delta_x = x2 - x1;
    let delta_y = y2 - y1;

    let k = delta_y / delta_x;
    let b = y1 - k * x1;

    let Vec2(x, y) = *point;

    (k * x + b - y).abs() < EPS
}

pub fn is_point_in_circle(point: &Vec2, circle: &Circle) -> bool {
    let Circle { center, radius } = circle;
    (point - center).magnitude_sqr() - radius.powi(2) < EPS
}

pub fn is_point_in_rectangle(point: &Vec2, rectangle: &Rectangle2D) -> bool {
    let (Vec2(min_x, min_y), Vec2(max_x, max_y)) = rectangle.min_max_points();

    // TODO: normal comparison for Vec2
    point.x <= max_x && point.y <= max_y && point.x >= min_x && point.y >= min_y
}
