mod vec2;
mod primitives;
mod intersect;

use vec2::Vec2;

fn main() {
    let t1 = Vec2::create(1_f64, 2_f64);
    let t2 = Vec2::create(1_f64, 2_f64);
    println!("Addition \n t1: {:#?} \n t2: {:#?}", t1, t2);
    println!("Test1 : {:#?}", &t1 + &t2);
    println!("\n");

    let t1 = Vec2::create(1_f64, 2_f64);
    let t2 = Vec2::create(1_f64, 2_f64);
    println!("Subtraction \n t1: {:#?} \n t2: {:#?}", t1, t2);
    println!("Test1 : {:#?}", &t1 - &t2);
    println!("\n");

    let t1 = Vec2::create(1_f64, 2_f64);
    let t2 = Vec2::create(1_f64, 2_f64);
    println!("Scalar product \n t1: {:#?} \n t2: {:#?}", t1, t2);
    println!("Test1 : {:#?}", &t1 ^ &t2);
    println!("\n");

    let t1 = Vec2::create(1_f64, 2_f64);
    let t2 = Vec2::create(1_f64, 2_f64);
    println!("Elementwise product \n t1: {:#?} \n t2: {:#?}", t1, t2);
    println!("Test1 : {:#?}", &t1 * &t2);
    println!("\n");


    let mut t1 = Vec2::create(1_f64, 2_f64);
    let t2 = Vec2::create(1_f64, 2_f64);
    println!("Addition with assignment \n t1: {:#?} \n t2: {:#?}", t1, t2);
    t1 += &t2;
    println!("Test1 : {:#?}", t1);
    println!("\n");

//    t1 == t2;
    let mut t1 = Vec2::create(1_f64, 2_f64);
    let t2 = Vec2::create(1_f64, 2_f64);
    println!("Subtraction with assignment \n t1: {:#?} \n t2: {:#?}", t1, t2);
    t1 -= &t2;
    println!("Test1 : {:#?}", t1);
    println!("\n");

    let t1 = Vec2::create(-1_f64, 2_f64);
    println!("Magnitude  \n t1: {:#?}", t1);
    println!("Test1 : {:#?}", t1.magnitude());
    println!("\n");


}

