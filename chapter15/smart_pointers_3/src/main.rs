use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    prog1();
    prog2();
    prog3();
    prog4();
} // end main()

fn prog1() {
    let x: Box<String> = Box::new("Hello, world!".to_string());

    let a: Box<String> = x;
    // This will cause an error!
    // let b: Box<String> = x;
} // end prog1()

fn prog2() {
    let x: Rc<String> = Rc::new("Hello, world!".to_string());

    let a: Rc<String> = Rc::clone(&x);
    println!("Count: {}", Rc::strong_count(&x));
    let b: Rc<String> = Rc::clone(&x);
    println!("Count: {}", Rc::strong_count(&x));
    {
        let c: Rc<String> = Rc::clone(&x);
        println!("Count: {}", Rc::strong_count(&x));
    }
    println!("Count: {}", Rc::strong_count(&x));
    println!("a = {}\nb = {}", a, b);
} // end prog2()

fn prog3() {
    let x: RefCell<Rc<String>> = RefCell::new(Rc::new(String::new()));

    assign(&x);

    println!("{}", *x.borrow());
}

fn assign(rf: &RefCell<Rc<String>>) {
    let st: Rc<String> = Rc::new(String::from("Hello, world!"));

    *rf.borrow_mut() = Rc::clone(&st);
}

fn prog4() {
    let mut x: Rc<String> = Rc::new(String::new());

    assign_2(&mut x);

    println!("{}", x);
}

fn assign_2(rf: &mut Rc<String>) {
    let st: Rc<String> = Rc::new(String::from("Hello, world!"));

    *rf = Rc::clone(&st);
}