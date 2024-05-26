fn main() {
    let y = {
        let x = 3;
        x + 1 // this line doesn’t have a semicolon at the end
              // If you add a semicolon to the end of an expression,
              // you turn it into a statement, and it will then
              // not return a value
    };
    // print_labeled_measurement(x, 'h');
    let result = print_labeled_measurement(y, 'h');
    println!("{result}");
}

fn print_labeled_measurement(value: i32, unit_label: char) -> bool {
    println!("The measurement is: {value}{unit_label}");
    true // return the last expression implicitly
}
