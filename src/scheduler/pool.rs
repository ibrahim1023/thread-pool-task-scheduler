use crossbeam::channel::{ unbounded, Sender };
use tokio::runtime::Runtime;

use crate::tasks::{ Task, AsyncTask };
use crate::scheduler::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Task>,
    rt: Runtime,
}

impl ThreadPool {
    pub fn new(size: usize, rt: Runtime) -> ThreadPool {
        let (sender, receiver) = unbounded();

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone(), rt.handle().clone()));
        }

        ThreadPool { workers, sender, rt }
    }

    pub fn execute(&self, task: Task) {
        self.sender.send(task).unwrap();
    }

    pub fn execute_async(&self, async_task: AsyncTask) {
        self.rt.spawn(async_task.run());
    }

    pub fn shutdown(self) {
        for worker in self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.join().unwrap();
        }
    }
}
