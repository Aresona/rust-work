enum Shape<T> {
    Square { line: T },
    Circle { radius: T },
    Triangle { a: T, b: T, c: T },
}

fn area<T: Copy + Into<f64>>(shape: Shape<T>) -> f64 {
    match shape {
        Shape::Circle { radius } => {
            let r: f64 = radius.into();
            std::f64::consts::PI * r * r
        }
        Shape::Square { line } => {
            let l: f64 = line.into();
            l * l
        }
        Shape::Triangle { a, b, c } => {
            let a: f64 = a.into();
            let b: f64 = b.into();
            let c: f64 = c.into();
            let p = (a + b + c) / 2.0;
            (p * (p - a) * (p - b) * (p - c)).sqrt()
        }
    }
}

fn main() {
    let triangle = Shape::Triangle { a: 2, b: 3, c: 4 };
    let circle = Shape::Circle { radius: 4.2 };
    let square = Shape::Square { line: 3 };
    let triangle_area = area(triangle);
    let square_area = area(square);
    let circle_area = area(circle);
    println!("triangle area is {}", triangle_area);
    println!("square area is {}", square_area);
    println!("circle area is {}", circle_area);
}
