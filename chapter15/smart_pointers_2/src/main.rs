fn main() {
    prog1();
    prog2();
    prog3();
    prog4();
} // end main()

fn prog1() {
    let x: Box<i32> = Box::new(5);

    println!("The value of x is {}", x);
} // end prog1()

fn prog2() {
    let x: i32 = -12;
    let y: Box<i32> = Box::new(x);

    println!("{} and {}", x, y);

    assert_eq!(x, *y);
} // end prog2()

struct MyBox<T> {
    value: T
} // end struct MyBox

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox { value }
    } // end new()
} // end impl MyBox

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    } // end deref()
} // end impl std::ops::Defef for MyBox

fn prog3() {
    let x: MyBox<String> = MyBox::new("Hello, world!".to_string());
    let y: String = String::from("Hello, world!");

    assert_eq!(*x, y);
} // end prog3()

fn prog4() {
    let x: MyBox<String> = MyBox::new("Hello, world!".to_string());

    println!("{}", *x);
} // end prog4()