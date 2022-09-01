use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::<Worker>::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        (0..size).for_each(|id| workers.push(Worker::new(id as u32, Arc::clone(&receiver))));

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let msg = Message::Job(Box::new(f));
        self.sender.send(msg).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.workers
            .iter()
            .for_each(|_| self.sender.send(Message::Terminate).unwrap());

        self.workers.iter_mut().for_each(|worker| {
            println!("Shutting down worker {}", worker.id);

            worker.thread.take().unwrap().join().unwrap();
        });
    }
}

struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        Self {
            id,
            thread: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::Job(job) => {
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} stop working", id);
                        break;
                    }
                };
            })),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Job(Job),
    Terminate,
}
