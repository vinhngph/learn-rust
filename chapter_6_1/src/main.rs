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

enum IpAddress {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{home:#?}");

    let loopback = IpAddress(String::from("::1"));
    println!("{loopback:#?}");
}
