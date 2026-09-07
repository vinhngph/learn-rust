#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum can have method
impl Message {
    fn call(&self) {
        //
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{home:#?}");

    let loopback = IpAddress::V6(String::from("::1"));
    println!("{loopback:#?}");

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

    println!("some_number={some_number:?} some_char={some_char:?} absent_number={absent_number:?}");

    let success: Result<i32, String> = Ok(42);
    let error: Result<i32, String> = Err("something went wrong.".to_string());
    println!("success={success:?} error={error:?}");
}
