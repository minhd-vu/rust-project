pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        // We use the FnOnce for the closure trait because that's what thread::spawn uses.
        F: FnOnce() + Send + 'static,
    {
    }
}
