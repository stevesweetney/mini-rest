use crate::looping::loop_breaks;
use std::sync::Mutex;
use std::thread;

pub fn main_loop(
    start_after_seconds: u64,
    min_work_time: u64,
    max_work_time: u64,
    rest_time: u64,
    volume: f32,
) {
    let is_paused = Mutex::new(false);
    let should_quit = Mutex::new(false);

    thread::scope(|s| {
        let t = s.spawn(|| {
            if let Err(e) = loop_breaks(
                start_after_seconds,
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
                    let mut guard = is_paused.lock().unwrap();
                    if *guard {
                        println!("Unpausing");
                        *guard = false;
                        t.thread().unpark();
                    } else {
                        println!("Pausing");
                        *guard = true;
                    }
                }
                "q" => {
                    println!("Quitting");
                    *should_quit.lock().unwrap() = true;
                    break;
                }
                _ => println!("Invalid command"),
            }

            buffer.clear();
        }
    });
}
