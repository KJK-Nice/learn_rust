#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other_point: &Point) -> f64 {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = other_point.x;
        let y2 = other_point.y;
        ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt()
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Square size 3: {:#?}", Rectangle::square(3));

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
        println!("The rectangle has area is {}", rect1.area());
    }

    let p1 = Point {x: 2.0, y: 2.0};
    let p2 = Point {x: 6.0, y: 6.0};
    println!("Distance from p1 to p2: {}", p1.distance(&p2));
    println!("Distance from p2 to p1: {}", p2.distance(&p1));
}
