// Vectors, Hasmaps, Casting, if-let, while-let and result enum
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
enum Example {
    Fval(f64),
    Ival(i32),
    Text(String),
}

fn main() {
    let mut v = vec![2, 3, 4, 9, 11, 12];
    v.push(13);
    // let mut v = Vec::new();
    // v.push(2);
    // v.push(3);
    // v.push(9);
    // v.push(11);
    // v.push(12);
    println!("{:?} {} {}", &v, v.len(), v.capacity());
    println!("{:?}", v.pop());

    let r = vec![
        Example::Fval(10.2),
        Example::Ival(2),
        Example::Text(String::from("There")),
    ];
    println!("{:?}", r);

    let mut hm = HashMap::new();
    hm.insert(String::from("Random"), 12);
    hm.insert(String::from("Numbers"), 14);

    for (i, k) in &hm {
        println!("{}: {}", i, k);
    }

    match hm.get(&String::from("Random")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }

    hm.remove(&String::from("strings"));
    println!("map size {}", hm.len());

    let s1 = Some('c');
    match s1 {
        Some(i) => println!("{}", i),
        _ => {}
    }

    if let Some(i) = s1 {
        println!("{}", i);
    }

    let mut s = Some(0);
    while let Some(i) = s {
        if i > 19 {
            println!("Quit");
            s = None
        } else {
            println!("{}", i);
            s = Some(i + 2);
        }
    }

    let f = 24.456;
    let i = f as u8;
    println!("{} {}", f, i);

    let f = File::open("Cargo.toml");

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There's a problem {:?}", error);
        }
    };
}
