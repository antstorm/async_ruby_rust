use rutie::Thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use tokio::runtime::{Builder, Runtime};

pub type Callback = Box<dyn FnOnce() + Send + 'static>;

pub enum Command {
    RunCallback(Callback),
    Shutdown,
}

pub struct AsyncRuntime {
    pub tokio: Runtime,
    pub callback_tx: Sender<Command>,
    callback_rx: Receiver<Command>,
}

impl AsyncRuntime {
    pub fn new(thread_count: u8) -> Self {
        let (tx, rx): (Sender<Command>, Receiver<Command>) = channel();
        let runtime =
            Builder::new_multi_thread()
                .worker_threads(thread_count.into())
                .enable_all()
                .thread_name("core")
                .build()
                .expect("Unable to start a runtime");

        AsyncRuntime { tokio: runtime, callback_tx: tx, callback_rx: rx }
    }

    // Call this from a Ruby thread
    pub fn run_callback_loop(&self) {
        let unblock = || {
            self.callback_tx.send(Command::Shutdown {}).expect("Unable to close callback loop");
        };

        while let Ok(msg) = Thread::call_without_gvl(|| self.callback_rx.recv(), Some(unblock)) {
            match msg {
                Command::RunCallback(callback) => callback(),
                Command::Shutdown => break,
            }
        };
    }
}
