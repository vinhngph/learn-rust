#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rec2: &Rectangle) -> bool {
        self.width >= rec2.width && self.height >= rec2.height
    }

    // associated functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 12,
        height: 4,
    };

    let rectangle2 = Rectangle {
        width: 11,
        height: 3,
    };

    println!("The area of {rectangle1:#?} is {}", rectangle1.area());
    println!(
        "Can rectangle1 hold rectangle2? {}",
        rectangle1.can_hold(&rectangle2)
    );

    let square = Rectangle::square(12);
    println!("The square is: {square:#?}");

    let area1 = rectangle1.area();
    let area2 = Rectangle::area(&rectangle1);
    assert_eq!(area1, area2);
}
