fn main() {
    get_len();
    println!("=====");
    change_str();
}

fn get_len() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &str) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change_str() {
    let mut s = String::from("hello");

    // If you add two or more references for one mutable var, it will report error
    // let r1 = &mut s;
    // let r2 = &mut s;

    // If you add different references incuding mutable and immutable, error will also be shown
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // But if you use it once, it will be dropped
    // from the last used time, at least earlier than
    // the r3 create
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("r3: {}", r3);

    change(&mut s);
    println!("origin s changed: {}", s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
