use integration_error_testing;

#[test]
fn add_function_works() {
    let a: usize = 2;
    let b: usize = 3;

    let result: usize = integration_error_testing::add(a, b);

    assert_eq!(result, 5);
}