use dirs::home_dir;
use ssh_config::SSHConfig;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

fn main() {
    let config_path = args()
        .collect::<Vec<String>>()
        .get(1)
        .map(|x| PathBuf::from(x))
        .unwrap_or_else(|| {
            let mut p = home_dir().unwrap();
            p.extend(Path::new(".ssh/config"));
            p
        });
    println!("Parsing {}...", config_path.display());
    // open file
    let mut file = match File::open(config_path.as_path()) {
        Ok(f) => f,
        Err(err) => panic!("Could not open file {}: {}", config_path.display(), err),
    };
    // Read
    let mut config = String::new();
    if let Err(err) = file.read_to_string(&mut config) {
        panic!("Could not read file: {}", err);
    }
    // Parse
    let config = match SSHConfig::parse_str(config.as_str()) {
        Ok(c) => c,
        Err(err) => panic!("Could not parse ssh configuration: {:?}", err),
    };
    println!("{:?}", config);
}
