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

impl PartialEq<Vec2> for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        let Vec2(self_x, self_y) = self;
        let Vec2(other_x, other_y) = self;


        (self_x - other_x).abs() < std::f64::EPSILON
            && (self_y - other_y).abs() < std::f64::EPSILON
    }

    fn ne(&self, other: &Vec2) -> bool {
        unimplemented!()
    }
}


#[cfg(test)]
mod test {
    use super::Vec2;

    #[test]
    pub fn sum() {
        let t1 = Vec2::create(1_f64, 2_f64);
        let t2 = Vec2::create(1_f64, 2_f64);
        println!("Addition \n t1: {:#?} \n t2: {:#?}", t1, t2);
        println!("Test1 : {:#?}", &t1 + &t2);
        assert_eq!(&t1 + &t2, Vec2(2_f64, 4_f64))
    }

    #[test]
    pub fn subtraction() {
        let t1 = Vec2::create(1_f64, 2_f64);
        let t2 = Vec2::create(1_f64, 2_f64);
        println!("Subtraction \n t1: {:#?} \n t2: {:#?}", t1, t2);
        println!("Test1 : {:#?}", &t1 - &t2);
        assert_eq!(&t1 - &t2, Vec2(0_f64, 0_f64));
    }

    #[test]
    pub fn scalar_product() {
        let t1 = Vec2::create(1_f64, 2_f64);
        let t2 = Vec2::create(1_f64, 2_f64);
        println!("Scalar product \n t1: {:#?} \n t2: {:#?}", t1, t2);
        println!("Test1 : {:#?}", &t1 ^ &t2);
        assert_eq!( &t1 ^ &t2, 5_f64);
    }

    #[test]
    pub fn elementwise_product() {
        let t1 = Vec2::create(1_f64, 2_f64);
        let t2 = Vec2::create(1_f64, 2_f64);
        println!("Elementwise product \n t1: {:#?} \n t2: {:#?}", t1, t2);
        println!("Test1 : {:#?}", &t1 * &t2);
        assert_eq!(&t1 * &t2, Vec2::create(1_f64, 4_f64));
    }


    #[test]
    pub fn assignment_with_addition() {
        let mut t1 = Vec2::create(1_f64, 2_f64);
        let t2 = Vec2::create(1_f64, 2_f64);
        println!("Addition with assignment \n t1: {:#?} \n t2: {:#?}", t1, t2);
        t1 += &t2;
        println!("Test1 : {:#?}", t1);
        assert_eq!(t1, Vec2(2_f64, 4_f64));
    }

    #[test]
    pub fn assignment_with_subtraction() {
        let mut t1 = Vec2::create(1_f64, 2_f64);
        let t2 = Vec2::create(1_f64, 2_f64);
        println!("Subtraction with assignment \n t1: {:#?} \n t2: {:#?}", t1, t2);
        t1 -= &t2;
        println!("Test1 : {:#?}", t1);
        assert_eq!(t1, Vec2::create(0_f64, 2_f64));
    }

    #[test]
    pub fn magnituted() {
        let t1 = Vec2::create(-1_f64, 2_f64);
        println!("Magnitude  \n t1: {:#?}", t1);
        println!("Test1 : {:#?}", t1.magnitude());
        assert_eq!(t1.magnitude(), ((-1_f64).powi(2) + 2_f64.powi(2)).sqrt())
    }
}