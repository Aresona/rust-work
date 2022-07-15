use std::convert::From;
use std::ops::{Add, Div, Mul, Sub};
enum Shape<T> {
    Square { line: T },
    Circle { radius: T },
    Triangle { a: T, b: T, c: T },
}

fn area<
    T: Mul<Output = f64>
        + Copy
        + From<f64>
        + Sub<T, Output = f64>
        + Add<T, Output = f64>
        + Div<T, Output = f64>,
>(
    shape: Shape<T>,
) -> f64 {
    match shape {
        // Shape::Circle { radius } => {
        //     let radius: f64 = radius.into();
        //     std::f64::consts::PI * radius * radius
        // }
        Shape::Square { line } => (line * line).into(),
        Shape::Circle { radius } => (radius * radius).into(),
        // Shape::Triangle { a, b, c } => a * a * a,
        Shape::Triangle { a, b, c } => {
            let semiperimeter: f64 = (a + b + c).into() / 2.0;
            semiperimeter.sqrt()
            // (semiperimeter.into()
            //     * (semiperimeter.into() - a.into())
            //     * (semiperimeter.into() - b.into())
            //     * (semiperimeter.into() - c.into()))
            // .sqrt()
        }
    }
}
fn main() {
    let triangle = Shape::Triangle::<f64> {
        a: 1.0,
        b: 3.1,
        c: 2.3,
    };
    let circle = Shape::Circle { radius: 2.1 };
    let square = Shape::Square { line: 3.3 };
    let triangle_area = area(triangle);
    let square_area = area(square);
    let circle_area = area(circle);
    println!("triangle area is {}", triangle_area);
    println!("square area is {}", square_area);
    println!("circle area is {}", circle_area);
}
