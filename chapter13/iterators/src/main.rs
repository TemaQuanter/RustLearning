fn main() {
    prog1();
    prog2();
    prog3();
} // end main()

fn prog1() {
    let vc: Vec<&str> = vec!["Alex", "Mary", "John", "Rust"];

    let it = vc.iter();

    for i in it {
        println!("{}", i);
    } // end for
} // end prog1()

fn prog2() {
    let vc: Vec<i32> = vec![32, 23, 88, 3894, 92, 9, -1, -313, 2];

    let total: i32 = vc.iter().sum();

    println!("Sum: {}", total);
} // end prog2()

fn prog3() {
    #[derive(PartialEq)]
    enum Subject {
        Mathematics,
        Programming,
        Chemistry,
        Biology,
        Astronomy,
        History,
        SocialScience,
        Geography,
    } // end enum Subjects

    impl std::fmt::Display for Subject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                Subject::Mathematics => write!(f, "Mathematics"),
                Subject::Programming => write!(f, "Programming"),
                Subject::Chemistry => write!(f, "Chemistry"),
                Subject::Biology => write!(f, "Biology"),
                Subject::Astronomy => write!(f, "Astronomy"),
                Subject::History => write!(f, "History"),
                Subject::SocialScience => write!(f, "SocialScience"),
                Subject::Geography => write!(f, "Geography"),
            } // end match
        } // end fmt
    } // end impl Display for Subject

    struct Student {
        id: u32,
        first_name: String,
        last_name: String,
        favorite_subject: Subject,
    } // end struct Student

    impl Student {
        fn display(&self) {
            println!("Student ID: {}", self.id);
            println!("First name: {}", self.first_name);
            println!("Last name: {}", self.last_name);
            println!("Favorite subject: {}", self.favorite_subject);
        } // end display()
    } // end impl Student

    let students: Vec<Student> = vec![
        Student {
            id: 322,
            first_name: "Sahra".to_string(),
            last_name: "Murphy".to_string(),
            favorite_subject: Subject::Astronomy,
        },
        Student {
            id: 116,
            first_name: "Mary".to_string(),
            last_name: "Conor".to_string(),
            favorite_subject: Subject::Geography,
        },
        Student {
            id: 32,
            first_name: "Alex".to_string(),
            last_name: "Roche".to_string(),
            favorite_subject: Subject::Astronomy,
        },
        Student {
            id: 990,
            first_name: "Kaly".to_string(),
            last_name: "Monster".to_string(),
            favorite_subject: Subject::Astronomy,
        },
        Student {
            id: 35,
            first_name: "Lera".to_string(),
            last_name: "Karpova".to_string(),
            favorite_subject: Subject::Mathematics,
        },
        Student {
            id: 55,
            first_name: "Arina".to_string(),
            last_name: "Zarevskaya".to_string(),
            favorite_subject: Subject::Astronomy,
        },
    ]; // end students

    let astronomy_funs: Vec<Student> = students
        .into_iter()
        .filter(|std| std.favorite_subject == Subject::Astronomy)
        .collect();

    for i in astronomy_funs.iter() {
        println!();
        i.display();
        println!();
    } // end for
} // end prog3()
