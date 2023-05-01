fn main() {
    let n = 2;
    if n < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let c = true;
    let _n = if c { 50 } else { 70 };
    println!("value of n: {}", _n);

    let mut c = 0;
    loop {
        println!("{}", c);
        c += 1;
        if c >= 10 {
            break;
        }
    }

    let mut n = 10;
    while n >= 0 {
        println!("{}", n);
        n -= 1;
    }

    let _a = vec![10, 20, 30, 40];
    for i in 1..10 {
        println!("{}", i);
    }

    let x = 15;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    // can match pairs or put if else condition inside match statements

    // we are allowed to lable loops as well, and break loop with name
    // '_a: loop {
    //     println!("loop a");
    //     '_b: loop {
    //         println!("loop b");
    //         '_c: loop {
    //             println!("loop c");
    //             break '_b;
    //         }
    //     }
    // }
}
