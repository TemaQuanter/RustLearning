/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program illustrates the way enums work in Rust.
 *
 * @date 6 Apr 2023
 * @version 0.1
 */
use core::f32::consts::PI;

// Declaring enum templates.

enum Shape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    Triangle(f32, f32, f32),
    Square { width: f32 },
} // end enum Shape

// Implementations.

// This implementation represents simple geometric shapes.
//
impl Shape {
    // This returns the area of a shape.
    //
    fn area(&self) -> f32 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Triangle(side1, side2, side3) => {
                let p: f32 = (side1 + side2 + side3) / 2.0;

                (p * (p - side1) * (p - side2) * (p - side3)).sqrt()
            }
            Shape::Square { width } => width * width,
        } // end match
    } // end area()
} // end impl Shape

fn main() {
    // Declare variables.

    let tr: Shape = Shape::Triangle(13.0, 12.0, 17.0);
    let sq: Shape = Shape::Square { width: 32.0 };
    let cr: Shape = Shape::Circle { radius: 3.0 };
    let rc: Shape = Shape::Rectangle {
        width: 12.0,
        height: 14.0,
    }; // end Shape

    // Display the area of each shape.

    println!("The area of triangle is {}", tr.area());
    println!("The area of square is {}", sq.area());
    println!("The area of circle is {}", cr.area());
    println!("The area of rectangle is {}", rc.area());
} // end main()
