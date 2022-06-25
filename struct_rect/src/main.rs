#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of rect = {}", rect.area());
    println!("{:#?}", rect);
    let rect2 = Rectangle::square(15);
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    dbg!(rect2);
}
