use rand::{thread_rng, Rng};
use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;
use std::thread;
use std::time::Duration;

const CHIME: &[u8] =
    include_bytes!("../zapsplat_multimedia_alert_mallet_and_chime_positive_004_63862.mp3");

fn prepare_chime<'a>() -> rodio::Decoder<Cursor<&'a [u8]>> {
    let cursor = Cursor::new(CHIME);

    Decoder::new(cursor).expect("Chime SFX is not a valid audio file")
}

pub fn loop_breaks(
    start_after_seconds: u64,
    min_work_time: u64,
    max_work_time: u64,
    rest_time: u64,
) {
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to open default audio output stream");

    let mut rng = thread_rng();

    thread::sleep(Duration::from_secs(start_after_seconds));
    println!("Starting timer!");

    loop {
        stream_handle
            .play_raw(prepare_chime().convert_samples())
            .expect("Failed to play chime SFX");
        thread::sleep(Duration::from_secs(rest_time));
        stream_handle
            .play_raw(prepare_chime().convert_samples())
            .expect("Failed to play chime SFX");

        thread::sleep(Duration::from_secs(
            rng.gen_range(min_work_time..=max_work_time),
        ));
    }
}
