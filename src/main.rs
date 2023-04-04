use clap::Parser;

mod core;
mod error;
mod looping;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Time in seconds to rest
    #[arg(short, long, default_value_t = 10)]
    rest_time: u64,

    /// Start timer after N seconds
    #[arg(short, long, default_value_t = 300)]
    start_n: u64,

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
    #[arg(short = 'v', long, default_value_t = 1.0)]
    volume: f32,
}

fn main() {
    let args = Args::parse();

    if args.min_work_time > args.max_work_time {
        let message = format!(
            "Lower bound, {}, must be less than or equal to upper bound, {}",
            args.min_work_time, args.max_work_time
        );
        panic!("{}", message);
    }

    if args.volume > 1.0 || args.volume < 0.0 {
        panic!("Volume must be in the range [0.0, 1.0]");
    }

    core::main_loop(
        args.start_n,
        args.min_work_time,
        args.max_work_time,
        args.rest_time,
        args.volume,
    );
}
