use remotefs::client::ssh::{SftpFs, SshOpts};
use remotefs::fs::Metadata;
use remotefs::RemoteFs;
use std::env;
use std::io::Cursor;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        panic!("Usage: <ssh-host> <port> <username> <password> [cycles]");
    }
    let hostname = &args[1];
    let port = args.get(2).map(|x| x.parse::<u16>().ok().unwrap()).unwrap();
    let username = &args[3];
    let password = &args[4];
    let cycles = args
        .get(5)
        .map(|x| x.parse::<usize>().ok().unwrap())
        .unwrap_or(10);
    env_logger::init();
    let mut client: SftpFs = SshOpts::new(hostname)
        .port(port)
        .username(username)
        .password(password)
        .into();
    assert!(client.connect().is_ok());
    // transfer
    let dest = Path::new("/tmp/test.bin");
    // Loop over cycles
    for n in 0..cycles {
        println!("Cycle {:02}", n + 1);
        let buffer = vec![0; 67108864]; // 64MB
        let buffer = Box::new(Cursor::new(buffer));
        let t_start = Instant::now();
        assert!(client
            .create_file(dest, &Metadata::default(), buffer)
            .is_ok());
        println!(
            "Cycle {:02} took {}us",
            n + 1,
            t_start.elapsed().as_micros()
        );
        assert!(client.remove_file(dest).is_ok());
    }
    assert!(client.disconnect().is_ok());
}
