use std::ops::RangeInclusive;
use clap::{Args, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct AppArgs {
    world: String,

    /// Run simulation on start.
    #[arg(short, default_value_t = false)]
    running: bool,

    /// Adjust the level of console output (0~4).
    /// The default verbosity is 1.
    /// [0: No output, 1: Error messages, 2: Error and warning messages,
    /// 3: Error, warning, and info messages, 4: Error, warning, info, and debug.]
    #[arg(short, long, value_parser = level_in_range)]
    verbose: Option<u8>,
}


const LEVEL_RANGE: RangeInclusive<usize> = 0..=4;

fn level_in_range(s: &str) -> Result<u8, String> {
    let level: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a number"))?;
    if LEVEL_RANGE.contains(&level) {
        Ok(level as u8)
    } else {
        Err(format!(
            "The value not in range {}-{}",
            LEVEL_RANGE.start(),
            LEVEL_RANGE.end()
        ))
    }
}

//
fn main() {
    let args = AppArgs::parse();

    #[cfg(feature = "rgz_sim")]
    {
        use rgz::sim;
        sim::run(
            args.world.to_string(),
            args.running,
            args.verbose.unwrap_or(1),
        );
    }
}
