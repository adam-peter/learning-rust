#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //fn area(rect: &Rectangle) -> u32 {
    fn area(&self) -> u32 {
        //short for (self: &Self)
        self.width * self.height
    }

    fn width(&self) -> bool {
        //we can create methods with same names as fields
        // -> declare fields private and create getters
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    //associated function - doesn't work with instances, but the whole Struct type
    //useful for constructors (::new()); like static methods
    fn square(a: u32) -> Self {
        Self {
            width: a,
            height: a,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 20,
    };

    //println!("Area of the rectangle: {}", area(&rect1));
    println!("Area of the rectangle: {}", rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(50);
    println!("{:?}", rect3);
}
