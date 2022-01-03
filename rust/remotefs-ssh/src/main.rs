use remotefs::client::ssh::{ScpFs, SftpFs, SshOpts};
use remotefs::fs::Metadata;
use remotefs::RemoteFs;
use std::env;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use std::time::{Duration, Instant};

const BUFSIZE: usize = 65535;

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
    // sftp
    let client: SftpFs = SshOpts::new(hostname)
        .port(port)
        .username(username)
        .password(password)
        .into();
    run_test("SFTP", client, cycles);
    // scp
    let client: ScpFs = SshOpts::new(hostname)
        .port(port)
        .username(username)
        .password(password)
        .into();
    run_test("SCP", client, cycles);
}

fn run_test(name: &str, mut client: impl RemoteFs, cycles: usize) {
    let dest = Path::new("/tmp/test.bin");
    println!("TEST: {}", name);
    assert!(client.connect().is_ok());
    let mut avg = Duration::ZERO;
    // Loop over cycles
    for n in 0..cycles {
        let mut file =
            File::open("/tmp/data.bin").expect("You need to put a file /tmp/data.bin to run this");
        let file_size: u64 = file.seek(std::io::SeekFrom::End(0)).unwrap_or(0);
        // rewind
        file.seek(std::io::SeekFrom::Start(0))
            .expect("Could not rewind");
        let t_start = Instant::now();
        let mut writer = client
            .create(dest, &Metadata::default().size(file_size))
            .ok()
            .unwrap();
        let mut bytes: usize = 0;
        while bytes < (file_size as usize) {
            let mut buffer: [u8; BUFSIZE] = [0; BUFSIZE];
            let bytes_read = file.read(&mut buffer).expect("Could not read file");
            let mut delta = 0;
            while delta < bytes_read {
                delta += writer
                    .write(&buffer[delta..bytes_read])
                    .expect("failed to write stream");
            }
            bytes += bytes_read;
        }
        avg += t_start.elapsed();
        println!(
            "Cycle {:02} took {}us ({}ms)",
            n + 1,
            t_start.elapsed().as_micros(),
            t_start.elapsed().as_millis()
        );
        assert!(client.remove_file(dest).is_ok());
    }
    let avg = avg / (cycles as u32);
    println!(
        "Average time: {}us ({}ms)",
        avg.as_micros(),
        avg.as_millis()
    );
    assert!(client.disconnect().is_ok());
}
