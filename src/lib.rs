use std::thread;

pub struct ThreadPool {
    // TODO: Vec of Workers, JoinHandler
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        // TODO: Spawn workers and put into Vec

        Self {}
    }
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,
    {
        // TODO: Handling ThreadPool execution, usage ThreadPool.execute(|| { handle_connection(); });
    }
}

struct Workers {
    // TODO:
}
