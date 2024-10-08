mod scheduler;
mod tasks;

use tasks::{ Task, AsyncTask };
use scheduler::ThreadPool;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();

    let pool = ThreadPool::new(4, rt);

    let task1 = Task::new(|| {
        println!("Task 1 executed");
    });

    let async_task = AsyncTask::new(async {
        println!("Async task executed");
    });

    pool.execute(task1);
    pool.execute_async(async_task);

    pool.shutdown();
}
