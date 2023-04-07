use restaurant::restaurant::{front_of_house::*, back_of_house};

fn main() {
    println!("Hello, world!");

    greet_guests();

    back_of_house::cook(&String::from("Pancakes"));
} // end main()
