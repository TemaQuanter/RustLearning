pub fn sum(a: i32, b: i32) -> i32 {
    a + b
} // end add()

pub fn dif(a: i32, b: i32) -> i32 {
    a - b
} // end dif()

pub fn greet_user(username: &str) -> String {
    format!("Hi, {}", username)
} // end greet_user()

pub fn password_check(psw: &str) {
    if psw == "cucumber" {
        panic!("The password is correct!");
    } // end if

    panic!("The password is incorrect!");
} // end password_check()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works_properly() {
        let a: i32 = 2;
        let b: i32 = 3;

        let c: i32 = sum(a, b);

        assert_eq!(c, 5);
    } // end addition_works_properly()

    #[test]
    fn substraction_works_properly() {
        let a: i32 = 2;
        let b: i32 = 3;

        let c: i32 = dif(a, b);

        assert_eq!(c, -1);
    } // end substraction_workds_properly()

    #[test]
    #[ignore]
    fn greeting_works() {
        let username: String = String::from("Alex");

        let result = greet_user(username.as_str());

        assert!(
            result.contains(username.as_str()),
            "The is an issue with the username.\nExpected username: {}\nFound greeting: {}",
            username,
            result
        );
    } // end greeting_works()

    #[test]
    #[should_panic(expected="correct")]
    fn correct_password_works() {
        let psw: &str = "cucumber";

        password_check(psw);
    } // end correct_password_works()

    #[test]
    #[should_panic(expected="incorrect")]
    fn incorrect_password_does_not_work() {
        let psw: &str = "notcucumber";

        password_check(psw);
    } // end incorrect_password_does_not_work()
} // end mod tests
