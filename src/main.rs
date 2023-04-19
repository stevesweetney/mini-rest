use clap::Parser;

mod core;

const ABOUT: &str = "A minimal looping timer. Work for a random period of time, then rest for a fixed period of time. Repeat. Press 'p' to pause/unpause, 'q' to quit.";

#[derive(Parser, Debug)]
#[command(author, version, about=Some(ABOUT), long_about = None)]
struct Args {
    /// Time in seconds to rest
    #[arg(short, long, default_value_t = 10)]
    rest_time: u64,

    /// Time to add to first work period
    #[arg(short, long, default_value_t = 300)]
    additional_start_time: u64,

    /// Minimum time in seconds to work.
    /// Lower bound for random work time
    #[arg(short = 'l', long, default_value_t = 150)]
    min_work_time: u64,

    /// Maximum time in seconds to work.
    /// Upper bound for random work time
    #[arg(short = 'u', long, default_value_t = 270)]
    max_work_time: u64,

    /// Volume of the chime. Valid values are
    /// in the range [0.0, 1.0]
    #[arg(short = 'v', long, default_value_t = 0.3)]
    volume: f32,
}

fn main() {
    let args = Args::parse();

    assert!(
        args.min_work_time <= args.max_work_time,
        "Lower bound, {}, must be less than or equal to upper bound, {}",
        args.min_work_time,
        args.max_work_time
    );
    assert!(args.rest_time > 0, "Rest time must be greater than 0");
    assert!(
        args.min_work_time > 0,
        "Min work time must be greater than 0"
    );
    assert!(
        !(args.volume > 1.0 || args.volume < 0.0),
        "Volume must be in the range [0.0, 1.0]"
    );

    core::main_loop(
        args.additional_start_time,
        args.min_work_time,
        args.max_work_time,
        args.rest_time,
        args.volume,
    );
}
