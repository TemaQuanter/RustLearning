/// Unsafe Rust

fn main() {
    prog1();
    prog2();
    prog3();
} // end main()

fn prog1() {
    let mut num: i32 = 5;

    let ptr1: *const i32 = &num as *const i32;
    let ptr2: *mut i32 = &mut num as *mut i32;

    let address: usize = 0x123edfusize;

    let _ptr_to_address: *mut usize = address as *mut usize;

    unsafe {
        println!("The data located in ptr1 is {}", *ptr1);
        *ptr2 = 7;
        println!("The data located in ptr1 is {}", *ptr1);
    } // end unsafe
} // end prog1()

unsafe fn dangerous() {
    let mut sentence: &str = "Hello, world";

    let ptr1: *const &str = &sentence;
    let ptr2: *mut &str = &mut sentence;

    println!("The initial sentence is \"{}\"", *ptr1);
    *ptr2 = "Hi everyone!";
    println!("The sentence after a change is \"{}\"", *ptr1);
} // end dangerous()

fn prog2() {
    unsafe {
        dangerous();
    } // end unsafe
} // end prog2();

extern "C" {
    fn add(a: i32, b: i32) -> i32;
} // end extern C

fn prog3() {
    let a: i32 = 7;
    let b: i32 = -2;

    let result: i32 = unsafe {
        add(a, b)
    }; // end usafe

    println!("{} + {} = {}", a, b, result);
}
