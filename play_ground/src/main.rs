use std::mem;
fn main() {
    println!("Hello, Tensor! I like rust");
    // i8, u8, i16, u16, i32, u32, i64, u64
    // f32, f64
    let _a = 1 + 20;
    let _s = 30 - 20;
    let _m = 5 * 10;
    let _b: bool = true;
    let _c: char = 'z';
    let _d = 4 / 6;
    let _t: (i32, f64, char) = (42, 6.12, 'j');

    // arrays are zero indexed
    let xs: [i32; 5] = [4, 5, 6, 7, 78];
    println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));

    // tuple inside tuple
    let t1 = (1, 'a', false);
    let t2 = (2, t1);
    println!("{} {:#?}", t1.0, t2); // use {:#?} for pretty print

    // print long tuples can cause error
    let _t = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
    // unable to print this tuple, println!("{:#?}", t);

    let xs1: [i32; 5] = [4, 5, 6, 7, 8];
    let ys1 = &xs[2..4]; //slice of an array from [start_index, end_index)
    println!("{:?} {:?}", xs1, ys1);

    // playing with strings
    let _s = "String"; // this is not a string but a slice of string with datatype &str
    let _s1 = "String".to_string(); // now this is a string
    let _s2 = String::from("String"); // or the other way of doing this
    let ss = &_s2[0..4];
    println!("{}", ss);

    // functions that doesn't return anything, that returns empty tuple
    let h = String::from("Hello, ");
    let w = String::from("World!");
    println!("{}", h + &w);
    // h + w is not valid, because the + operator in string uses (self, &str)
}
