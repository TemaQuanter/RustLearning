mod simple_math;

use crate::simple_math::{add, sub, geometry::Shape};
fn main() {
    // Declare variables.

    let a: i32 = 1;
    let b: i32 = 2;
    let c = add(a, b);

    let tr: Shape = Shape::Triangle(4.0, 3.0, 5.0);
    let rc: Shape = Shape::Rectangle {
        width: 32.0,
        height: 3.0,
    };
    let cr: Shape = Shape::Circle { radius: 13.35 };

    // Display the result of arithmetic operation.

    println!("{} + {} = {}", a, b, c);

    // Display some information about geometric shapes.

    println!(
        "Triangle's perimeter: {}\nTriangle's area: {}",
        tr.perimeter(),
        tr.area()
    );
    println!(
        "Rectangle's perimeter: {}\nRectangle's area: {}",
        rc.perimeter(),
        rc.area()
    );
    println!(
        "Circle's perimeter: {}\nCircle's area: {}",
        cr.perimeter(),
        cr.area()
    );
} // end main()
