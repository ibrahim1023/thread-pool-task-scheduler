use crossbeam::channel::Receiver;
use std::thread;
use tokio::runtime::Handle;
use crate::tasks::Task;

pub struct Worker {
    pub id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Receiver<Task>, handle: Handle) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(task) = receiver.recv() {
                println!("Worker {} got a task; executing...", id);

                handle.block_on(async {
                    task.run();
                });
            }
        });

        Worker { id, thread: Some(thread) }
    }

    pub fn join(self) -> thread::Result<()> {
        if let Some(thread) = self.thread { thread.join() } else { Ok(()) }
    }
}
