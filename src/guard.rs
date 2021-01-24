use std::{process, thread};

pub struct PanicGuard;

impl PanicGuard {
    pub fn new() -> Self {
        Self
    }
}

impl Drop for PanicGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            log::error!("Thread panic");
            process::exit(1);
        }
    }
}
