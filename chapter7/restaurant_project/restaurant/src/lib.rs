pub mod restaurant {
    pub mod front_of_house {
        pub fn greet_guests() {
            println!("Hello, dear guests!");
        } // end greet_guests()
        pub fn suggest_drinks() {
            println!("We have:\nCoke\nApple juice\nWatter");
        } // end suggest_drinks()
    } // end mod front_of_house
    pub mod back_of_house {
        pub fn cook(dish: &String) {
            println!("Cooking {}", dish);
        } // end cook()
    } // end mod back_of_house
} // end mod restaurant