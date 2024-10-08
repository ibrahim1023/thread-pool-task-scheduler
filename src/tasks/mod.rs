use std::pin::Pin;
use std::future::Future;
pub struct Task {
    job: Box<dyn FnOnce() + Send + 'static>,
}

impl Task {
    pub fn new<F>(f: F) -> Task where F: FnOnce() + Send + 'static {
        Task { job: Box::new(f) }
    }

    pub fn run(self) {
        (self.job)();
    }
}

pub struct AsyncTask {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl AsyncTask {
    pub fn new<F>(f: F) -> AsyncTask where F: Future<Output = ()> + Send + 'static {
        AsyncTask {
            future: Box::pin(f),
        }
    }

    pub async fn run(self) {
        self.future.await;
    }
}
