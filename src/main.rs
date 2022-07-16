use std::convert::From;
use std::ops::{Add, Div, Mul, Sub};
enum Shape<T> {
    Square { line: T },
    Circle { radius: T },
    Triangle { a: T, b: T, c: T },
}

// fn area<
//     T: Mul<Output = T> + Copy + From<f64> + Sub<Output = T> + Add<Output = T> + Div<Output = T>,
// >(
//     shape: Shape<T>,
// ) -> f64 {
//     match shape {
//         Shape::Circle { radius } => {
//             let temp = radius * radius;
//             std::f64::consts::PI * temp.into()
//         }
//         Shape::Square { line } => (line * line).into(),
//         Shape::Triangle { a, b, c } => {
//             let semiperimeter = (a + b + c) / 2.0;
//             (semiperimeter.into()
//                 * (semiperimeter.into() - a.into())
//                 * (semiperimeter.into() - b.into())
//                 * (semiperimeter.into() - c.into()))
//             .sqrt()
//         }
//     }
// }

fn area<
    T: Mul<Output = f64>
        + Copy
        + Sub<f64, Output = f64>
        + Sub<T, Output = f64>
        + Mul<f64, Output = f64>
        + Add<Output = T>
        + From<f64>,
>(
    shape: Shape<T>,
) -> f64 {
    match shape {
        Shape::Circle { radius } => {
            let temp = radius * radius;
            std::f64::consts::PI * temp
        }
        Shape::Square { line } => line * line,
        Shape::Triangle { a, b, c } => {
            let semiperimeter = (a + b + c) * 0.5;
            (semiperimeter * (semiperimeter - a) * (semiperimeter - b) * (semiperimeter - c)).sqrt()
        }
    }
}

fn main() {
    let triangle = Shape::Triangle {
        a: 1.0,
        b: 3.1,
        c: 2.3,
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
