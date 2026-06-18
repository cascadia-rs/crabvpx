use std::sync::{Mutex, Condvar};


// Opaque struct for semaphore
pub struct Semaphore {
    mutex: Mutex<u32>,
    cond: Condvar,
}

impl Semaphore {
    pub fn new(value: u32) -> Self {
        Semaphore {
            mutex: Mutex::new(value),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut count = self.mutex.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn signal(&self) {
        let mut count = self.mutex.lock().unwrap();
        *count += 1;
        self.cond.notify_one();
    }
}





