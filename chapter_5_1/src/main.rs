struct User {
    username: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;

fn main() {
    let user1 = User {
        username: String::from("someone"),
        sign_in_count: 3,
    };

    let user2 = User {
        username: String::from("abc"),
        ..user1
    };

    println!("{}", user1.username);
    println!("{} {}", user2.username, user2.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black={}", black.0);
    println!("origin={}", origin.1);

    let subject = AlwaysEqual;
}
