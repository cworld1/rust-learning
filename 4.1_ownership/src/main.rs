fn main() {
    scooped();
    println!("=====");
    push_str();
    println!("=====");
    clone_str();
    println!("=====");
    returns1();
    println!("=====");
    returns2();
}

fn scooped() {
    {
        // s is not valid here, it’s not yet declared
        let s = "scooped"; // s is valid from this point forward
        println!("{s}");
        // do stuff with s
    } // this scope is now over, and s is no longer valid
}

fn push_str() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}

fn clone_str() {
    let s1 = String::from("hello");
    // If you don't use ".clone()", it will be a "move" action
    // let s2 = s1;
    // And this action of reading s1 will make a wrong build
    // println!("{s1}");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn returns1() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}", s); // reported wrong

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn returns2() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.
fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    // let some_string = String::from("yours"); // some_string comes into scope
    // some_string
    // some_string is returned
    // and moves out to the calling function
    String::from("yours")
}
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
