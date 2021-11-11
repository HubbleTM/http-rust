use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: i32,
    thread: Option<JoinHandle<()>>,
}

pub struct ThreadPool {
    sender: Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id as i32, Arc::clone(&receiver)));
        }

        return ThreadPool {
            sender,
            workers,
        };
    }

    pub fn execute(&self, f: Job) {
        self.sender.send(Message::NewJob(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: i32, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv().unwrap();
            match msg {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} finished.", id);
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}