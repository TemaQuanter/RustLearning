use std::{
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
        Arc,
        Mutex
    },
    thread,
    thread::JoinHandle,
}; // end use std::thread

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
} // end ThreadPool

impl ThreadPool {
    /// This function creates a new ThreadPool.
    ///
    /// # Panics
    ///
    /// The `new` panics if the `size` equals to 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver): (Sender<Job>, Receiver<Job>) = mpsc::channel();

        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        } // end for

        ThreadPool { workers, sender }
    } // end new()

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let job: Job = Box::new(f);

        self.sender.send(job).unwrap();
    } // end execute()
} // end impl ThreadPool

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
} // end struct Worker

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread: JoinHandle<()> = thread::spawn(move || loop {
            let job: Job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job, executing...");

            job();
        });

        Worker {
            id,
            thread
        } // end Worker
    } // end new()
} // end impl Worker
