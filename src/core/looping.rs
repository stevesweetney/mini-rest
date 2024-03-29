use super::error::{self, AudioError};
use rand::{thread_rng, Rng};
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const CHIME: &[u8] = include_bytes!("../../sounds/timer-sound.mp3");

fn prepare_chime<'a>() -> error::Result<rodio::Decoder<Cursor<&'a [u8]>>> {
    let cursor = Cursor::new(CHIME);

    Decoder::new(cursor).map_err(|_| AudioError)
}

pub fn loop_breaks(
    additional_start_time: u64,
    min_work_time: u64,
    max_work_time: u64,
    rest_time: u64,
    volume: f32,
    is_paused: &AtomicBool,
    should_quit: &AtomicBool,
) -> error::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().map_err(|_| AudioError)?;

    let sink = Sink::try_new(&stream_handle).map_err(|_| AudioError)?;
    sink.set_volume(volume);

    let mut rng = thread_rng();

    println!("Starting timer! Press 'p' to pause, 'q' to quit");

    let first_work_period = Duration::from_secs(rng.gen_range(min_work_time..=max_work_time))
        + Duration::from_secs(additional_start_time);
    let mut wake_at = Instant::now() + first_work_period;

    'outer: loop {
        if Instant::now() > wake_at {
            wake_at =
                Instant::now() + Duration::from_secs(rng.gen_range(min_work_time..=max_work_time));
        }

        while Instant::now() < wake_at {
            if is_paused.load(Ordering::Acquire) {
                println!("Paused. Press 'p' to unpause");
                thread::park();
                continue 'outer;
            }

            if should_quit.load(Ordering::Acquire) {
                break 'outer;
            }

            thread::sleep(Duration::from_millis(100));
        }

        sink.append(prepare_chime()?);
        sink.sleep_until_end();

        thread::sleep(Duration::from_secs(rest_time));

        sink.append(prepare_chime()?);
        sink.sleep_until_end();
    }

    Ok(())
}
