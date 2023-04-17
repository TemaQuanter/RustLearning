use std::ops::Add;
use std::fmt::{Display, Result, Formatter};

struct Gram(u32);

struct Kilogram(u32);

impl Add<Kilogram> for Gram {
    type Output = Gram;

    fn add(self, rhs: Kilogram) -> Self::Output {
        Gram(self.0 + (rhs.0 * 1000))
    } // end add()
} // end impl Add<Kilogram> for Gram

impl Display for Gram {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    } // end fmt()
} // end impl Display for Gram

impl Display for Kilogram {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    } // end fmt()
} // end impl Display for Kilogram


fn main() {
    let a: Gram = Gram(30);
    let b: Kilogram = Kilogram(15);

    let c: Gram = a + b;

    println!("30 grams + 15 kilograms = {} grams", c);
} // end main()
