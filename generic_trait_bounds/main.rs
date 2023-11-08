use std::ops::{Add};
use std::time::{Duration};

// The args to add() can accept any type that implements std::ops::Add
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let floats = add(1.2, 3.4);     // with floating point values
    let ints = add(10, 20);     // with integer values

    // represetns a duration between two points in time
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);    // std::time::Duration does not implement the std::fmt::Display
                                    // train, fall back to requesting std::fmt::Debug
}
