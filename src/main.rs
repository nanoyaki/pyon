use clap::Parser;
use clap_num::number_range;
mod pyon;

/// Helper function to validate the command-line numeric argument
fn valid_bunny_count(s: &str) -> Result<u16, String> {
    number_range(s, 0, 65535)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Print ASCII bunnies to your terminal
struct Args {
    /// How many bunnies to print
    #[arg(short = 'c', long = "count", default_value_t = 1, value_parser = valid_bunny_count)]
    count: u16,

    /// Are you literally this bunny?
    #[arg(short = 'l', long = "literally", action)]
    literally: bool
}

/// Prints ASCII bunnies depending on command-line parameters
fn main() {
    let args = Args::parse();

    pyon::print_bunnies(args.literally, args.count);
}
