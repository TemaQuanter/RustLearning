/// This function addes one to the provided argument.
/// 
/// # Example
/// 
/// ```
/// use add_one::add_one;
/// 
/// fn main() {
///     let x: i32 = 6;
///     let y: i32 = add_one(x);
/// 
///     assert_eq!(y, 7);
/// }
/// ```
pub fn add_one(num: i32) -> i32 {
    num + 1
} // end add_one()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_works() {
        let x: i32 = 7;

        assert_eq!(add_one(x), 8);
    } // end add_one_works()
} // end mod test