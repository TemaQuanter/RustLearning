/// This function substracts 1 from the provided value.
/// 
/// # Example
/// 
/// ```
/// use sub_one::sub_one;
/// 
/// fn main() {
///     let x: i32 = 7;
/// 
///     assert_eq!(sub_one(x), 6);
/// }
/// ```
/// 
pub fn sub_one(num: i32) -> i32 {
    num - 1
} // end sub_one()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_one_works() {
        let x: i32 = -3;

        assert_eq!(sub_one(x), -4);
    } // end sub_one_works()
} // end mod tests
