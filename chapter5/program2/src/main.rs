/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program demonstrates a way to implement methods
 *        for structs.
 *
 * @date 5 Apr 2023
 * @version 0.1
 */

// Declare structure templates.

// This structure represents a triangle.
//
#[derive(Debug)]
struct Triangle {
    side1: f32,
    side2: f32,
    side3: f32,
} // end struct Triangle

// Implementatios.

// Implementation for Triangle structure.
//
impl Triangle {
    // This function initializes a Triangle.
    //
    fn new(side1: f32, side2: f32, side3: f32) -> Self {
        Self {
            side1,
            side2,
            side3,
        }
    } // end init()

    // This function sets a Triangle.
    //
    fn set(&mut self, side1: f32, side2: f32, side3: f32) {
        *self = Triangle {
            side1,
            side2,
            side3,
        }; // end Triangle
    } // end set()

    // This function calculates a perimeter of a Triangle.
    //
    fn perimeter(&self) -> f32 {
        self.side1 + self.side2 + self.side3
    } // end perimeter()

    // This function calculates an area of a Triangle.
    //
    fn area(&self) -> f32 {
        // This is a half of a Triangle perimeter.
        let p: f32 = (self.side1 + self.side2 + self.side3) / 2.0;

        (p * (p - self.side1) * (p - self.side2) * (p - self.side3)).sqrt()
    } // end area()
} // end impl Triangle

fn main() {
    // Declare variables.

    let tr1: Triangle;
    let mut tr2: Triangle;
    let tr3: Triangle;

    // Initialize Triangles.

    tr1 = Triangle {
        side1: 4.0,
        side2: 5.0,
        side3: 3.0,
    }; // end Triangle

    tr2 = Triangle {
        side1: 0.0,
        side2: 0.0,
        side3: 0.0,
    }; // end Triangle

    tr3 = Triangle::new(32.0, 45.0, 27.0);

    tr2.set(11.0, 12.0, 7.0);

    // Display some information about Triangles.

    println!("Triangle tr1: {:#?}", tr1);
    println!("Some information about this triangle");
    println!("Perimeter: {}\nArea: {}", tr1.perimeter(), tr1.area());

    println!();

    println!("Triangle tr2: {:#?}", tr2);
    println!("Some information about this triangle");
    println!("Perimeter: {}\nArea: {}", tr2.perimeter(), tr2.area());

    println!();

    println!("Triangle tr3: {:#?}", tr3);
    println!("Some information about this triangle");
    println!("Perimeter: {}\nArea: {}", tr3.perimeter(), tr3.area());
} // end main()
