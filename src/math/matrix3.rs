use std::ops::{Index, Mul};

pub struct Matrix3 {
    array: [f64; Self::TOTAL],
}

trait Matrix {
    const NA: usize;
}

impl Matrix3 {
    pub const N: usize = 3;
    pub const TOTAL: usize = Self::N * Self::N;

    pub fn create(_00: f64, _01: f64, _02: f64,
                  _10: f64, _11: f64, _12: f64,
                  _20: f64, _21: f64, _22: f64) -> Matrix3 {
        Self::from_array([
            _00, _01, _02,
            _10, _11, _12,
            _20, _21, _22
        ])
    }

    pub fn from_array(array: [f64; Self::TOTAL]) -> Matrix3 {
        Matrix3 { array }
    }

    pub fn create_zero() -> Matrix3 {
        Matrix3 { array: [0_f64; Self::TOTAL] }
    }

    pub fn transpose(&self) -> Matrix3 {
        let mut new_array = [0_f64; Self::TOTAL];
        for i in 0..Self::N {
            for j in 0..Self::N {
                new_array[Self::N * j + i] = self.array[Self::N * i + j];
            }
        }
        Self::from_array(new_array)
    }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        if i > Self::N || j > Self::N {
            panic!("Index out of matrix range")
        }

        &self.array[Self::N * i + j]
    }
}

impl Mul<f64> for &Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f64) -> Matrix3 {
        let mut new_array = [0_f64; Matrix3::TOTAL];
        for (idx, x) in self.array.iter().enumerate() {
            new_array[idx] = *x;
        }

        Matrix3::from_array(new_array)
    }
}
