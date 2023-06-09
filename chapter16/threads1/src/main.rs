use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    prog1();
    prog2();
    prog3();
    prog4();
    prog5();
} // end main()

fn prog1() {
    let spawned_thread: thread::JoinHandle<()> = thread::spawn(|| {
        for i in 0..10 {
            println!("Hi from a thread! {{{}}}", i);
            thread::sleep(Duration::from_millis(10));
        } // end for
    }); // end thread

    for i in 0..10 {
        println!("Hi from main! {{{}}}", i);
        thread::sleep(Duration::from_millis(10));
    } // end for

    spawned_thread.join().expect("The spawned thread failed");
} // end prog1()

fn prog2() {
    let vc: Vec<&str> = vec!["Alex", "Mary", "Irishka", "Arishka", "Toma"];

    let spawned_thread = thread::spawn(move || {
        println!("{:?}", vc);
    });

    spawned_thread.join().expect("The called thread failed");
} // end prog2()

fn prog3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("Hello, world!");

        tx.send(value).expect("The value sending failed");
    }); // end thread::spawn()

    let value = rx.recv().expect("The value should have been received");

    println!("Got the value: {}", value);
} // end prog3()

fn prog4() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let data: Vec<String> = vec![
            "Hi".to_string(),
            "from".to_string(),
            "a".to_string(),
            "thread".to_string(),
        ];

        for el in data {
            tx.send(el).expect("Failed to transfer data");
            thread::sleep(Duration::from_secs(1));
        } // end for
    }); // end thread::spawn()

    for data in rx {
        println!("Got: {}", data);
    } // end for
} // end prog4()

fn prog5() {
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    let mut transmitters: Vec<mpsc::Sender<String>> = Vec::new();
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for _ in 0..10 {
        transmitters.push(tx.clone());
    } // end for

    drop(tx);

    for (i, tm) in transmitters.into_iter().enumerate() {
        threads.push(thread::spawn(move || {
            for _ in 0..10 {
                tm.send(format!("Hi from transmitter number {}", i))
                    .expect("Failed to send data");
                thread::sleep(Duration::from_millis(50));
            } // end for
        }));
    } // end for

    for message in rx {
        println!("{}", message);
    } // end for

    for th in threads.into_iter() {
        th.join().expect("A thread failed");
    } // end for
} // end prog5()
