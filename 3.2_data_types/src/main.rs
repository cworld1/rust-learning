use std::io;

fn array_game() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn data_types() {
    let y: f32 = 3_003.2; // f32
    println!("y: {y}");

    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    // also can use this to get x:
    // let five_hundred = tup.0;
    println!("The value of tuple is: ({x}, {y}, {z})");

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("array[0]: {first}");
}

fn main() {
    data_types();
    println!("==========");
    array_game();
}
