use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}
pub struct PoolCreationError;
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id))
        }

        ThreadPool { workers }
    }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// The `build` function will return an error if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError);
        }

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id))
        }

        Ok(ThreadPool { workers })
    }

    pub fn execute<F>(&self, f: F)
    where
        // We use the FnOnce for the closure trait because that's what thread::spawn uses.
        F: FnOnce() + Send + 'static,
    {
    }
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
