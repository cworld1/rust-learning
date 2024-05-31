fn main() {
    let mut s = String::from("hello world");

    let (start, end) = first_word_pos(&s); // word will get the value 5
                                           // end still has the value 5 here, but there's no more string that
                                           // we could meaningfully use the value 5 with. word is now totally invalid!

    // println!("{}", &s[start..end]);
    for i in start..end {
        print!("{}", s.chars().nth(i).unwrap());
    }
    println!();

    println!("=====");

    let word = first_word(&s);
    println!("{}", word);

    s.clear(); // this empties the String, making it equal to ""

    println!("=====");

    test_array_slice();
}

fn first_word_pos(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (0, i);
        }
    }

    (0, s.len())
}

// fn first_word(s: &String) -> &str {
// It allows us to use the same function
// on both &String values and &str values.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return &s[0..i];
            // 0 can be droped
            return &s[..i];
        }
    }

    // &s[0..s.len()]
    // same they can be droped
    // &s[..]
    s
}

fn test_array_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
