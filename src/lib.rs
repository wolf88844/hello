use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

#[warn(dead_code)]
pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);
        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { threads, sender:Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down all workers.");
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread)=worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message{
                    Ok(job)=>{
                        println!("Worker {} got a job; executing.",id);
                        job();
                    }
                    Err(_)=>{
                        println!("Worker {} shutting down.", id);
                        break;
                    }
                }
            }
        });
        Worker { id, thread:Some(thread) }
    }
}
