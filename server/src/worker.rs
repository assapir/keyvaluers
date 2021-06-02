use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::thread_pool::Message;

pub struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("unable to lock mutex")
                .recv()
                .expect("Unable to receive message");

            match message {
                Message::NewJob(job) => {
                    println!("worker {}: processing job", id);

                    job();
                }
                Message::Terminate => {
                    println!("worker {}: shutting down", id);
                    break; // Exit the loop so it will not hang
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
