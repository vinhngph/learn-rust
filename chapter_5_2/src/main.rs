#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rectangle = Rectangle {
        width: 12,
        height: 4,
    };

    println!("The area = {}", area(&rectangle));
    println!("The rectangle: {}x{}", rectangle.width, rectangle.height);

    println!("The rectangle is: {rectangle:?}");
    println!("The rectangle is (with pretty output): {rectangle:#?}");

    dbg!(&rectangle);
}
