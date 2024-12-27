use std::f64::consts::PI;

// Generic structs
#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    height: T,
}

#[derive(Debug)]
struct Circle<T, U> {
    center_x: T,
    center_y: T,
    radius: U,
}

// Generic impl block
impl<T, U> Circle<T, U> {
    fn get_radius(&self) -> &U {
        return &self.radius;
    }
}

// Specialized impl block (Partial Specialization)
impl<T> Circle<T, f64> {
    fn get_area(&self) -> f64 {
        return PI * self.get_radius() * self.get_radius();
    }
}

// Specialized impl block (Full Specialization)
impl Circle<i32, f64> {
    fn get_center_manhattan_distance(&self) -> i32 {
        return self.center_x + self.center_y;
    }
}

// Generic functions
fn get_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn main() {
    let rect_i32 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect_f64 = Rectangle {
        width: 10.1,
        height: 20.2,
    };
    let circle = Circle {
        center_x: 3,
        center_y: 4,
        radius: 5.0,
    };

    println!("rect i32 = {:?}", rect_i32);
    println!("rect f64 = {:?}", rect_f64);
    println!("circle = {:?}", circle);
    println!("circle radius = {}", circle.get_radius());
    println!("circle area = {}", circle.get_area());
    println!(
        "circle manhattan distance = {}",
        circle.get_center_manhattan_distance()
    );
    println!("get_max(10, 20) is {}", get_max(10, 20));
    println!("get_max(100.0, 20.0) is {}", get_max(100.0, 20.0));
}
