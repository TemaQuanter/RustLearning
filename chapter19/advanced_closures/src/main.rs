fn main() {
    repeater(say_hi, 15);
    repeater(|| println!("Hello, world!"), 3);

    let result: i32 = get_math_op(add, 13, 5);
    let result2: i32 = get_math_op(|a: i32, b: i32| a - b, -3, 5);

    let result3: i32 = get_math_op(|a: i32, b: i32| {
        let mut l: i32 = 0;
        let mut r: i32 = a;

        let mut m: i32;

        while r - l > 1 {
            m = (l + r) / 2;

            if m * b > a {
                r = m;
            } else {
                l = m;
            } // end if
        } // end while

        l
    }, 17, 3);

    println!("{} {} {}", result, result2, result3);
} // end main()

fn repeater(func: fn(), times: u32) {
    for _ in 0..times {
        func();
    } // end for
} // end repeater()

fn say_hi() {
    println!("Hi!");
} // end say_hi()

fn get_math_op(func: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    func(a, b)
} // end call_add_func()

fn add(a: i32, b: i32) -> i32 {
    a + b
} // end add()
