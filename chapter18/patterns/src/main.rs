fn main() {
    prog1();
    prog2();
    prog3();
    prog4();
    prog5();
    prog6();
    prog7();
} // end main()

fn prog1() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(5);
    stack.push(1);
    stack.push(4);
    stack.push(3);

    while let Some(el) = stack.pop() {
        println!("{}", el);
    } // end for
} // end prog1()

fn print_coordinates(&(x, y): &(f32, f32)) {
    println!("The coordinates are {{{}, {}}}", x, y);
} // end print_coordinates()

fn prog2() {
    let point = (32.7, -1.2);

    print_coordinates(&point);
} // end prog2()

fn prog3() {
    let x: i32 = 10;

    match x {
        1..=5 => println!("From 1 to 5 inclusive"),
        6..=8 => println!("From 6 to 8 inclusive"),
        9 | 10 => println!("9 or 10"),
        _ => println!("Something else")
    } // end match
} // end prog3()

struct Person {
    first_name: String,
    last_name: String
} // end struct person

fn prog4() {
    let p1 = Person {
        first_name: String::from("Artem"),
        last_name: String::from("Mikheev")
    }; // end Person

    let Person { first_name: f_name, last_name: l_name } = p1;

    println!("{} {}", f_name, l_name);
} // end prog4()

fn prog5() {
    let ((x1, y1), (x2, y2)) : ((i32, i32), (i32, i32)) = ((32, -12), (-19, 200));

    match ((x1, y1), (x2, y2)) {
        (.., (-19, _)) => println!("The x2 is {}", x2),
        _ => println!("A segment")
    } // end match
} // end prog5()

fn prog6() {
    let x: Option<u32> = Some(13);

    match x {
        Some(x) if x % 2 == 0 => println!("x is even"),
        Some(_) => println!("x is odd"),
        None => ()
    } // end match
} // end prog6()

enum Shape {
    Triangle{ side1: i32, side2: i32, side3: i32 },
    Circle(i32)
} // end enum Shape

fn prog7() {
    let sh1: Shape = Shape::Triangle{ side1: 21, side2: 16, side3: 15 };

    match sh1 {
        Shape::Triangle { side1: just_side @ 0..=20, .. } => println!("Good triangle with the side {}", just_side),
        Shape::Triangle { side1: _, side2: just_side @ 10..=17, .. } => println!("Very good triangle with the side {}", just_side),
        _ => println!("Something else")
    } // end match
} // end prog7()
