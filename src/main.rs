use std::println;

fn main() {
    // By declaring the variable as mutable, it can be modified
    let mut my_number = 5;
    // It can be modified
    my_number = 10 + my_number;

    let number_type: i32 = 10;

    println!("Type number i32:{}", number_type);
    println!("My number is: {}", my_number);
    println!("Hello, world from Rust!")
}
