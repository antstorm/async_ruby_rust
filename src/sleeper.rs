use std::{thread, time};

pub fn sleep(time: u64) {
  let duration_ms = time::Duration::from_millis(time);
  println!("Sleeping...");
  thread::sleep(duration_ms);
  println!("Done!")
}
