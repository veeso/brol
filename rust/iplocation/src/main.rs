use std::{net::IpAddr, path::PathBuf};

use argh::FromArgs;

/// Lookup IP address in ip2location database
#[derive(FromArgs)]
struct CliArgs {
    /// path to ip2location database
    #[argh(option, short = 'd')]
    db: PathBuf,
    /// IP address to lookup
    #[argh(positional)]
    ip: IpAddr,
}

fn main() -> anyhow::Result<()> {
    let args: CliArgs = argh::from_env();
    let mut db = ip2location::DB::from_file(args.db)?;

    let lookup = db.ip_lookup(args.ip)?;
    let country = match lookup {
        ip2location::Record::LocationDb(rec) => rec.country.unwrap().short_name,
        ip2location::Record::ProxyDb(rec) => rec.country.unwrap().short_name,
    };

    println!("{country}");

    Ok(())
}
