use std::mem;
fn take(v: Vec<i32>) {
    println!("We took v: {}", v[10] + v[100]);
}
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

    // LEC-3 Memory Managment
    // in cpp, memory is allocated but not relased causing an application to consume memory
    // reducing the available memory for other applications and eventually causing the system to
    // page virtual memory to the hard drive slowing the application or crashing the application
    // than the computer's memory resource limit is reached.
    // Heap memory guys
    // - Attempting to free memory already freed
    // - Freeing memory that was not allocated
    // - attempting to read/write memory already freed.
    // - memory allocation error.
    // - reading/writing to memory out of bounds of dynamically allocated array.
    // Stack Memory errors
    // - Reading writing to memory out of bound
    // - function pointer corruption: invalid passing of function pointer and thus a bad call to a
    // function
    // Garabage collector - it is an algorithm (used by langauges like JAVA, C# etc) that finds the
    // free memory and releases it automatically

    // Rust uses ownership
    // variable is the owner of the memory, and when owner goes out of scope - the value is dropped
    // each piece can have one owner at a time.

    let s_temp = String::from("String");
    // let y = s_temp; // reference or ownership is changed from s to y, and s is no longer accessible
    let y = &s_temp; // but borrowing is allowed
    println!("{} {}", s_temp, y);

    let mut v = Vec::new();
    for i in 1..1000 {
        v.push(i);
    }
    take(v);
    println!("Finished!");

    let v = vec![4, 5, 6, 7, 4, 5, 6, 7, 4, 1, 2, 9, 0];
    for &i in &v {
        let r = count(&v, i);
        // v is borrwed and i is copied
        println!("{} is repeated {} times ", i, r);
    }
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}
