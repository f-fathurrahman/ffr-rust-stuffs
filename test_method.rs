struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // This is static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    // another static method
    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // this is an instance method
    // `&self` is a sugar for `self: &Self`, where `Self` is the type
    // of the caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ( (x1-x2)*(y1-y2) ).abs()
    }
}


fn main() {

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle area: {}", rectangle.area());
}
