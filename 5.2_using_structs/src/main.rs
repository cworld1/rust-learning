#[derive(Debug)] // State it can be used to transform to Debug
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // show the result of calculation and return
        height: 50,
    };
    // Use {:?} print debug infos
    // println!("rect1 is {:?}", rect1);
    //
    // Use # to make output with clearer format
    // println!("rect1 is {:#?}", rect1);
    //
    // Also can use dbg micro instead,
    // which can also show the specific position of code
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
