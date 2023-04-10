mod shapes;

use crate::shapes::*;

fn main() {
    let tr: Triangle = Triangle {
        side1: 4.0,
        side2: 5.0,
        side3: 3.0,
    }; // end Triangle

    let cr: Circle = Circle { radius: 12.0 }; // end Circle

    // Display the information about the shapes.

    display_shape(&tr);
    display_shape(&cr);
} // end main()
