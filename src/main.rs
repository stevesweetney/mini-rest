use clap::Parser;

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
    #[arg(short, long, default_value_t = 150)]
    min_work_time: u64,

    /// Maximum time in seconds to work.
    /// Upper bound for random work time
    #[arg(short, long, default_value_t = 270)]
    max_work_time: u64,
}

fn main() {
    let args = Args::parse();
    looping::loop_breaks(
        args.start_n,
        args.min_work_time,
        args.max_work_time,
        args.rest_time,
    );
}
