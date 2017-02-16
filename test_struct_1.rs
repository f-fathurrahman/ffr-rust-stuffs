use std::fmt;

struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // the `f` value implements the `Write` trait, which is what
        // the write! macro is expecting.
        write!(f, "point: ({},{})", self.x, self.y)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // the `f` value implements the `Write` trait, which is what
        // the write! macro is expecting.
        write!(f, "Rectangle ({},{})", self.p1, self.p2)
    }
}

fn main() {
    let point: Point = Point{ x: 0.3, y: 0.4};
    println!("point coordinates: ({},{})", point.x, point.y);
    println!("{}", point);

    // Destructure the point using a `let` binding
    let Point {x: my_x, y: my_y} = point;
    println!("my_x = {}, my_y = {}", my_x, my_y);

    let rectangle = Rectangle {
        p1: Point {x: my_y, y: my_x },
        p2: point,
    };
    println!("rectangle: {}", rectangle);

    // instantiate a unit struct
    let _nil = Nil;

    // instantiate a tuple struct
    let pair = Pair(1, 0.11);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("integer = {:?}, decimal = {:?}", integer, decimal);
}
