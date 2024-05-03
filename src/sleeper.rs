use crate::async_runtime::{AsyncRuntime, Callback, Command};
use std::{thread, time};

pub fn sleep_sync(runtime: &AsyncRuntime, time: u64) {
  let duration_ms = time::Duration::from_millis(time);
  println!("[RUST] Sync sleep");

  runtime.tokio.block_on(async {
    thread::sleep(duration_ms);
    println!("[RUST] Awake from sync sleep")
  });
}

pub fn sleep_async<F>(runtime: &AsyncRuntime, time: u64, callback: F) where F: FnOnce() + Send + 'static {
  let duration_ms = time::Duration::from_millis(time);
  let tx = runtime.callback_tx.clone();

  println!("[RUST] Async sleep");

  runtime.tokio.spawn(async move {
    thread::sleep(duration_ms);

    let callback: Callback = Box::new(move || callback());

    tx.send(Command::RunCallback(callback)).expect("Unable to send a callback");
    println!("[RUST] Awake from async sleep");
  });
}
