use std::thread;

pub(crate) struct ComThread {

}

impl ComThread {
    pub(crate) fn new() -> ComThread {
        ComThread {

        }
    }

    pub(crate) fn run(self) {
        thread::spawn(ComThread::worker);
    }

    fn worker() {
        loop {

        }
    }
}