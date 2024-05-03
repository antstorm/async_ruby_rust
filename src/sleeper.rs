use std::{thread, time};
use tokio::runtime::Runtime;

pub fn sleep(runtime: &Runtime, time: u64) {
  let duration_ms = time::Duration::from_millis(time);
  println!("Sleeping...");

  runtime.spawn(async move {
    thread::sleep(duration_ms);
    println!("Awake!")
  });

  println!("Done!")
}
