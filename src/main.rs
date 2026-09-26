fn main() {
    // al colorcarle mut la variable tiende a ser muteable y se puede modificar
    let mut my_number = 5;
    // se puede modificar
    my_number = 10 + my_number;
    println!("My number is: {}", my_number);

    println!("Hello, world from Rust!")
}
