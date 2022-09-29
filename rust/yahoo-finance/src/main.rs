use argh::FromArgs;
use chrono::{DateTime, Utc};
use yahoo_finance::history;

#[derive(FromArgs)]
#[argh(
    description = "Please, report issues to <https://github.com/veeso/bitpanda730>
Please, consider supporting the author <https://ko-fi.com/veeso>"
)]
pub struct Args {
    #[argh(option, short = 'f', description = "start date")]
    pub from: DateTime<Utc>,
    #[argh(option, short = 't', description = "end date")]
    pub to: DateTime<Utc>,
    #[argh(positional, description = "symbol to fetch")]
    pub symbol: String,
}

fn main() {
    let args: Args = argh::from_env();

    let data = history::retrieve_range(&args.symbol, args.from, Some(args.to))
        .expect("failed to fetch symbol");

    // print the date and closing price for each day we have data
    for bar in &data {
        println!(
            "On {} {} closed at ${:.2}",
            bar.timestamp.format("%b %e %Y"),
            args.symbol,
            bar.close
        )
    }
}
