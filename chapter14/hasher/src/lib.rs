/// This function accepts a parameter that could be
/// converted to `String` and then returns a hash
/// of that parameter.
/// 
/// # Example:
/// 
/// ## src/main.rs
/// ```
/// use hasher::*;
/// 
/// fn main() {
///     let password: &str = "This is a secret password";
/// 
///     let hs: u128 = hash(&password);
/// 
///     println!("The hash of \"{}\" is {}", password, hs);
/// 
///     assert_eq!(hs, 302015122871951274883946804534689327083);
/// } 
/// ```
/// ## Output
/// ```sh
/// The hash of "This is a secret password" is 302015122871951274883946804534689327083
/// ```
/// 
pub fn hash<T>(data: &T) -> u128 where T: ToString {
    // Declare variables.

    const P: u128 = 257;
    const M: u128 = 52348570210230357367492380506945223581;
    const LIMIT: u128 = u128::MAX / P;

    let st = data.to_string();

    let mut result: u128 = 0;
    let mut pw: u128 = P;

    // Calculate the hash of the data.

    for b in st.bytes() {
        result = result + ((b as u128) * pw) % M;
        pw = if pw < LIMIT { pw * P } else { P };
    } // end for

    std::cmp::max(u128::MAX - result, result)
} // end hash()

// This function sums up numbers safely.
// This function is needed to avoid errors with u128 overflow.
//
fn safe_sum(a: u128, b: u128) -> Result<u128, &'static str> {
    // Declare variables.

    // Calculate the maximum number that could be added,
    // so not to get integer overflow.

    let max_add: u128 = u128::MAX - a;

    // Check if that number is more or equal to the number
    // that should be added.

    if max_add < b {
        // The number is too large and cannot be safely added.
        // A danger for integer overflow.

        Err("Integer overflow")
    } else {
        Ok(a + b)
    } // end if

} // end safe_sum()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_works() {
        let data: &str = "Hello, world!";
        let hs: u128 = hash(&data);

        assert_eq!(hs, 340281654467088561928157055835406716534);
    } // end hash_works()

    #[test]
    #[should_panic(expected="Integer overflow")]
    fn safe_sum_works_panic() {
        let a: u128 = 340281654467088561928157055835406716534;
        let b: u128 = u128::MAX - a + 1;

        safe_sum(a, b).expect("Integer overflow");
    } // end safe_sum_works_panic()

    #[test]
    fn safe_sum_works_accepted() {
        let a: u128 = 340281654467088561928157055835406716534;
        let b: u128 = u128::MAX - a;

        safe_sum(a, b).expect("Integer overflow");
    } // end safe_sum_works_panic()
}
