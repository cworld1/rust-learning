struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );
    user1.email = String::from("anotheremail@example.com");
    print_user("User1", &user1);

    println!("=====");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    print_user("User2", &user2);

    println!("=====");

    // Tupple struct
    let black = Color(0, 0, 0);
    let origin = Point(100, 0, 0);
    println!("({}, {}, {})", black.0, black.1, black.2);
    println!("({}, {}, {})", origin.0, origin.1, origin.2);

    // Unit-like struct
    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        // username: username,
        // If the data has the same name with field, it can be written as
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(name: &str, user: &User) {
    print!("[{name}] {} ", user.username);
    println!("({})", if user.active { "active" } else { "deactive" });
    print!("Email: {}\t ", user.email);
    println!("Sign in count: {}", user.sign_in_count);
}
