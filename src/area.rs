use std::ops::{Add, Mul, Sub};
enum Shape<T> {
    Square { line: T },
    Circle { radius: T },
    Triangle { width: T, height: T },
}

fn area<T: Mul<Output = f64> + Copy + Sub<T, Output = f64> + Add<Output = T>>(
    shape: Shape<T>,
) -> f64 {
    match shape {
        Shape::Circle { radius } => {
            let temp = radius * radius;
            std::f64::consts::PI * temp
        }
        Shape::Square { line } => line * line,
        Shape::Triangle { width, height } => width * height * 0.5,
    }
}

fn main() {
    let triangle = Shape::Triangle {
        width: 2.3,
        height: 3.2,
    };
    let circle = Shape::Circle { radius: 4.1 };
    let square = Shape::Square { line: 3.2 };
    let triangle_area = area(triangle);
    let square_area = area(square);
    let circle_area = area(circle);
    println!("triangle area is {}", triangle_area);
    println!("square area is {}", square_area);
    println!("circle area is {}", circle_area);
}
