use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// Methods
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({} {}) and Area: {}",
            self.width,
            self.height,
            self.area(),
        )
    }
}

// Related Functions
fn area(obj: &Object) -> u32 {
    obj.width * obj.height
}

fn main() {
    println!("OLA");
    let o = Object {
        width: 35,
        height: 30,
    };
    println!(
        "{} x {} area: {} and using other way {}",
        o.width,
        o.height,
        area(&o),
        o.area()
    );

    let obj1 = Object::new(20, 20);
    println!(
        "{} x {} new area for obj1: {}",
        obj1.height,
        obj1.width,
        obj1.area()
    );
    println!("{}", o);
    println!("{:#?}", obj1);
}
