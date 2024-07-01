use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{

    pub fn new() -> Self {
        Self::build(None)
    }

    pub fn build(size: Option<usize>) -> ThreadPool {
        let thread_num = match size {
            Some(0) => 4,
            Some(threads) => threads,
            None => 4,
        };

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(thread_num);

        for id in 0..thread_num{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static,{
            let job = Box::new(f);

            self.sender.send(job).unwrap();
    }
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn( move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker {id, thread}
    }
}