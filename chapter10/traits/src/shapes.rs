use std::f32::consts::PI;

pub trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
} // end trait Shape

pub struct Triangle {
    pub side1: f32,
    pub side2: f32,
    pub side3: f32,
} // end struct Trialge

pub struct Circle {
    pub radius: f32,
} // end struct Circle

impl Shape for Triangle {
    fn area(&self) -> f32 {
        let p: f32 = self.perimeter() / 2.0;
        (p * (p - self.side1) * (p - self.side2) * (p - self.side3)).sqrt()
    } // end area()

    fn perimeter(&self) -> f32 {
        self.side1 + self.side2 + self.side3
    } // end perimeter
} // end impl Shape for Triangle

impl Shape for Circle {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    } // end area()

    fn perimeter(&self) -> f32 {
        2.0 * PI * self.radius
    } // end perimeter()
} // end impl Shape for Circle

pub fn display_shape(shp: &impl Shape) {
    println!("The perimeter of the shape is {}", shp.perimeter());
    println!("The area of the shape is {}", shp.area());
} // end display_shape()
