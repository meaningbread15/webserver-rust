use std::thread;
use std::{sync::{mpsc, Arc, Mutex}};

pub struct Threadpool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Threadpool {
    pub fn new(size: usize) -> Threadpool{
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Threadpool{ workers, sender }
    }
    pub fn execute<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} gets a job: executing");
                job();
            }
        });

        Worker { id, thread }
    }
}
