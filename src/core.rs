mod error;
mod looping;

use looping::loop_breaks;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub fn main_loop(
    additional_start_time: u64,
    min_work_time: u64,
    max_work_time: u64,
    rest_time: u64,
    volume: f32,
) {
    let is_paused = AtomicBool::new(false);
    let should_quit = AtomicBool::new(false);

    thread::scope(|s| {
        let t = s.spawn(|| {
            if let Err(e) = loop_breaks(
                additional_start_time,
                min_work_time,
                max_work_time,
                rest_time,
                volume,
                &is_paused,
                &should_quit,
            ) {
                eprintln!("Error: {}", e);
            }
        });

        let mut buffer = String::new();
        loop {
            if t.is_finished() {
                break;
            }
            std::io::stdin().read_line(&mut buffer).unwrap();

            match buffer.trim() {
                "p" => {
                    if is_paused.load(Ordering::Acquire) {
                        println!("Unpausing");
                        is_paused.store(false, Ordering::Release);
                        t.thread().unpark();
                    } else {
                        println!("Pausing");
                        is_paused.store(true, Ordering::Release);
                    }
                }
                "q" => {
                    println!("Quitting");
                    should_quit.store(true, Ordering::Release);
                    break;
                }
                _ => println!("Invalid command"),
            }

            buffer.clear();
            thread::sleep(Duration::from_millis(100));
        }
    });
}
