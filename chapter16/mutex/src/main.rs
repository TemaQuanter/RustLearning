use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    prog1();
    prog2();
    prog3();
} // end main()

fn prog1() {
    let m: Mutex<String> = Mutex::new(String::from("Hello, world!"));

    {
        let mut new_m = m.lock().expect("The variable is locked");

        *new_m = String::from("This is a new string:)");
    }

    println!("{}", m.lock().expect("The variable is locked"));
} // end prog1()

fn prog2() {
    let names: [String; 5] = [
        String::from("Alex"),
        String::from("Mary"),
        String::from("Arishka"),
        String::from("Toma"),
        String::from("Artem"),
    ];

    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    let mut txs: Vec<mpsc::Sender<String>> = Vec::new();

    for _ in 0..(names.len() - 1) {
        txs.push(tx.clone());
    } // end for

    txs.push(tx);

    for (i, tx) in txs.into_iter().enumerate() {
        let name: String = names[i].clone();
        thread::spawn(move || {
            for _ in 0..10 {
                thread::sleep(Duration::from_millis(50));
            } // end for

            tx.send(format!("{}", name))
                .expect("Failed to send the data through the channel");
        }); // end thread::spawn()
    } // end for

    for el in rx {
        println!("{}", el);
    } // end for
} // end prog2()

fn prog3() {
    let names: [String; 5] = [
        String::from("Alex"),
        String::from("Mary"),
        String::from("Arishka"),
        String::from("Toma"),
        String::from("Artem"),
    ];

    let winner: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for i in 0..names.len() {
        let name: String = names[i].clone();
        let rf: Arc<Mutex<String>> = Arc::clone(&winner);

        threads.push(thread::spawn(move || {
            for _ in 0..10 {
                thread::sleep(Duration::from_millis(50));
            } // end for

            let mut new_winner = rf.lock().expect("The variable is locked");

            *new_winner = name;
        })); // end thread::spawn()
    } // end for

    for th in threads.into_iter() {
        th.join().expect("A thread failed");
    } // end for

    println!(
        "The winner is ........ {}",
        *winner.lock().expect("The variable is locked!")
    );
} // end prog2()
