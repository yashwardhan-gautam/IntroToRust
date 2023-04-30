// ENUS AND OPTIONS
// #![allow(dead_code)]

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// #[derive(Debug)]
// enum Direction {
//     Up(Point),
//     Down(Point),
//     Left(Point),
//     Right(Point),
// }

// #[derive(Debug)]
// enum Keys {
//     UpKey(String),
//     DownKey(String),
//     LeftKey(String),
//     RightKey(String),
// }

// impl Direction {
//     fn match_direction(&self) -> Keys {
//         match *self {
//             Direction::Up(_) => Keys::UpKey(String::from("w")),
//             Direction::Down(_) => Keys::UpKey(String::from("s")),
//             Direction::Left(_) => Keys::UpKey(String::from("a")),
//             Direction::Right(_) => Keys::UpKey(String::from("d")),
//         }
//     }
// }

// impl Keys {
//     fn destruct(&self) -> &String {
//         match *self {
//             Keys::UpKey(ref s)
//             | Keys::DownKey(ref s)
//             | Keys::LeftKey(ref s)
//             | Keys::RightKey(ref s) => s,
//         }
//     }
// }

// fn main() {
//     let u = Direction::Up(Point { x: 0, y: 1 });
//     let k = u.match_direction();
//     println!("{:?}", k.destruct());
// }

enum Shape {
    Rectange { width: u32, height: u32 },
    Square(f64),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectange { width, height } => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * r * r,
        }
    }
}

// Option<T> has two type T or None
fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let r = Shape::Rectange {
        width: 10,
        height: 30,
    };
    let s = Shape::Square(10.1);
    let c = Shape::Circle(5.0);

    println!("{} {} {}", r.area(), s.area(), c.area());

    let res = division(5.0, 10.2);
    match res {
        Some(x) => println!("{:.5}", x),
        None => println!("Invalid division"),
    }
}
