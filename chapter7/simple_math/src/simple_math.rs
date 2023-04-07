pub fn add(a: i32, b: i32) -> i32 {
    a + b
} // end add()

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
} // end add()

pub mod geometry {
    use std::f32::consts::PI;

    pub enum Shape {
        Triangle(f32, f32, f32),
        Circle { radius: f32 },
        Square { width: f32 },
        Rectangle { width: f32, height: f32 },
    } // end enum Shape

    impl Shape {
        pub fn perimeter(&self) -> f32 {
            match self {
                Shape::Triangle(side1, side2, side3) => side1 + side2 + side3,
                Shape::Circle { radius } => 2.0 * PI * radius,
                Shape::Square { width } => 4.0 * width,
                Shape::Rectangle { width, height } => 2.0 * (width + height),
            } // end match
        } // end perimeter()

        pub fn area(&self) -> f32 {
            match self {
                Shape::Triangle(side1, side2, side3) => {
                    let p: f32 = (side1 + side2 + side3) / 2.0;
                    (p * (p - side1) * (p - side2) * (p - side3)).sqrt()
                }
                Shape::Circle { radius } => PI * radius * radius,
                Shape::Square { width } => width * width,
                Shape::Rectangle { width, height } => width * height,
            } // end match
        } // end area()
    } // end impl Shape
} // end mod Geometry
