struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_fit(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 200
    };
    let rect2 = Rectangle {
        width: 50,
        height: 100
    };   
    let rect3 = Rectangle {
        width: 300,
        height: 200
    };

    println!("Area of rect2: {}", rect2.area());

    println!("Can fit rect2: {}", rect1.can_fit(&rect2));
    println!("Can fit rect3: {}", rect1.can_fit(&rect3));

}
