fn main() {
    prog1();
    prog2();
    prog3();
    prog4();
    prog5();
} // end main()

fn prog1() {
    let x: i32 = 3;
    let y: i32 = -1;

    let add = || x + y;

    println!("The sum of the variables is {}", add());
} // end prog1()

fn prog2() {
    let vc: Vec<String> = vec![String::from("Alex"), String::from("Artem")];

    let printer = |v: Vec<String>| {
        for el in &v {
            println!("{}", el);
        } // end for
    }; // end printer||

    printer(vc);
} // end prog2()

fn prog3() {
    let x: i32 = 3;
    let y: i32 = -1;

    let sum = |x: i32, y: i32| -> i32 { x + y }; // end sum||

    println!("{} + {} = {}", x, y, sum(x, y));
} // end prog3()

fn prog4() {
    let mut vc: Vec<i32> = Vec::new();

    let mut fill_vector = |filler: i32, times: usize| {
        for _ in 0..times {
            vc.push(filler);
        } // end for
    }; // end fill_vector||

    fill_vector(32, 12);
    println!("{:?}", vc);
} // end prog4()

fn prog5() {
    let s1: String = String::from("Hello, world!");
    let s2: String = String::from("Hello, humanity!");

    let printer = |s1: String, s2: &String| {
        println!("{}\n{}", s1, s2);
    }; // end printer()

    println!("{}", s1);
    printer(s1, &s2);
    println!("{}", s2);

    // Error: The value is moved.
    // println!("{}", s);

    // Just to revise arrays.

    let arr1: [i32; 5] = [7, 3, -2, 3, 0];
    let arr2: [&str; 3] = ["Alex", "Mary", "John"];
    let arr3: [u32; 5] = [3; 5];

    println!("{:?}", arr1);
    println!("{:?}", arr2);
    println!("{:?}", arr3);
} // end prog5()
