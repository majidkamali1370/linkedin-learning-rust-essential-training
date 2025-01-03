use std::f64::consts::PI;
use std::fmt;

struct Circle {
    center: (f64, f64),
    radius: f64,
}

#[derive(PartialEq, PartialOrd)]
struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    width: f64,
    height: f64,
}

trait HasArea {
    // fn get_area(&self) -> f64; // <- No default impl

    // Default impl
    fn get_area(&self) -> f64 {
        return 0.0;
    }
}

impl HasArea for Circle {
    fn get_area(&self) -> f64 {
        return PI * self.radius * self.radius;
    }
}

impl HasArea for Rectangle {
    fn get_area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl HasArea for Triangle {} // <- Uses default impl

// Restrict get_share_area with trait bounds (T: HasArea)
fn get_shape_area<T: HasArea>(s: &T) -> f64 {
    return s.get_area();
}

// following line is equivalent to where statement after that
// fn compare_and_display<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_display<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn main() {
    let c = Circle {
        center: (2.0, 2.0),
        radius: 5.0,
    };
    let r1 = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let r2 = Rectangle {
        width: 5.0,
        height: 10.0,
    };

    println!("Circle area is {}", get_shape_area(&c));
    println!("Rectangle area is {}", get_shape_area(&r1));
    println!("R1 == R2 ? {}", r1 == r2); // <- Because of #[derive(PartialEq)] trait
    println!("R1 == R2 ? {}", r1 > r2); // <- Because of #[derive(PartialOrd)] trait
    compare_and_display(1.0, 1);
    compare_and_display(1.1, 1);
}
