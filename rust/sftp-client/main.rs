/**
 *
 *
 *           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *                   Version 2, December 2004
 *
 *  Copyright (C) 2020 Christian Visintin
 *
 *  Everyone is permitted to copy and distribute verbatim or modified
 *  copies of this license document, and changing it is allowed as long
 *  as the name is changed.
 *
 *             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
 *
 *   0. You just DO WHAT THE FUCK YOU WANT TO.
*/
// Dependencies
extern crate chrono;
extern crate rpassword;
extern crate ssh2;

// Includes
use chrono::prelude::*;
use ssh2::{Session, Sftp};
use std::env;
use std::fs::OpenOptions;
use std::io;
use std::io::*;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::exit;

struct SftpClient {
    client: Sftp,
    wrkdir: PathBuf,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check args len
    if args.len() < 2 {
        eprintln!("Usage: {} <address> [port]", args.get(0).unwrap());
        exit(255);
    }
    let address: String = args.get(1).unwrap().clone();
    let port: u16 = match args.get(2) {
        Some(p) => p.parse::<u16>().unwrap(),
        None => 22,
    };
    // Create session
    println!("Connecting to {}:{}", address, port);
    let tcp = TcpStream::connect(format!("{}:{}", address, port)).unwrap();
    // Create session
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    println!("Connection established");
    loop { // Until authentication succeds
        // Ask for username
        print!("Username: ");
        // Flush
        io::stdout().flush().unwrap();
        let mut username = String::new();
        let _ = io::stdin().read_line(&mut username);
        let password: String = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
        // Trim
        trim_newline(&mut username);
        // Try to authenticate
        match session.userauth_password(username.as_str(), password.as_str()) {
            Ok(_) => {
                println!("Authentication succeded");
                break
            },
            Err(err) => eprintln!("Authentication failed: {}", err)
        }
    }
    // Print banner
    println!("{}", session.banner().unwrap_or(""));
    // Set blocking to true
    session.set_blocking(true);
    // Prepare SFTP
    let mut sftp: SftpClient = SftpClient::new(&session);
    loop {
        // Read stdin
        let command: String = read_cmd();
        let argv: Vec<&str> = command.split(" ").collect();
        if argv.len() == 0 {
            continue;
        }
        // Match command
        let uppercase_cmd: String = String::from(argv[0]).to_uppercase();
        match uppercase_cmd.as_str() {
            "CWD" => sftp.cwd(argv),
            "DEL" => sftp.del(argv),
            "GET" => sftp.get(argv),
            "HELP" => sftp.usage(),
            "LS" => sftp.ls(argv),
            "MKDIR" => sftp.mkdir(argv),
            "MOV" => sftp.mov(argv),
            "PUT" => sftp.put(argv),
            "PWD" => sftp.pwd(),
            "RMDIR" => sftp.rmdir(argv),
            "QUIT" => break,
            "" => continue,
            _ => {
                eprintln!("Unknown command '{}'", uppercase_cmd);
                sftp.usage();
            }
        }
    }
    // Close session
    let _ = session.disconnect(None, "mandi", None);
    exit(0);
}

/// ### trim_newline
///
/// Trim newlines from string

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

/// ### read_cmd
///
/// Read user command

fn read_cmd() -> String {
    print!(">> ");
    // Flush
    io::stdout().flush().unwrap();
    // Read
    let mut command = String::new();
    let _ = io::stdin().read_line(&mut command);
    // Trim
    trim_newline(&mut command);
    // Return
    command
}

// @! SFTP library

impl SftpClient {
    pub fn new(session: &Session) -> SftpClient {
        // Get wrkdir
        let sftp_cli: Sftp = session.sftp().unwrap();
        let currdir: PathBuf = sftp_cli.realpath(PathBuf::from(".").as_path()).unwrap();
        SftpClient {
            client: sftp_cli,
            wrkdir: currdir,
        }
    }

    /// ### cwd
    ///
    /// Change working directory
    pub fn cwd(&mut self, argv: Vec<&str>) {
        if argv.len() < 2 {
            eprintln!("Missing argument");
            self.usage();
            return;
        }
        // Check if is relative
        let path = PathBuf::from(argv[1]);
        self.wrkdir = self.get_abs_path(path.as_path());
    }

    /// ### del
    ///
    /// Delete provided file
    pub fn del(&self, argv: Vec<&str>) {
        if argv.len() < 2 {
            eprintln!("Missing argument");
            self.usage();
            return;
        }
        let path = PathBuf::from(argv[1]);
        let path: PathBuf = self.get_abs_path(path.as_path());
        // Delete
        if let Err(err) = self.client.unlink(path.as_path()) {
            eprintln!("Could not remove file '{}': {}", path.display(), err);
        }
    }

    /// ### get
    ///
    /// get file from remote
    pub fn get(&self, argv: Vec<&str>) {
        if argv.len() < 2 {
            eprintln!("Missing argument");
            self.usage();
            return;
        }
        let remote = PathBuf::from(argv[1]);
        let remote: PathBuf = self.get_abs_path(remote.as_path());
        let local: PathBuf = match argv.get(2) {
            Some(arg) => PathBuf::from(arg),
            None => PathBuf::from(remote.as_path().file_name().unwrap()),
        };
        // Open remote file
        match self.client.open(remote.as_path()) {
            Ok(mut rhnd) => {
                let file_size: u64 = rhnd.seek(SeekFrom::End(0)).unwrap_or(0);
                // rewind
                if let Err(err) = rhnd.seek(SeekFrom::Start(0)) {
                    eprintln!("Could not rewind file: {}", err);
                    return;
                }
                // Create local file
                match OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(local.as_path())
                {
                    Ok(mut lhnd) => {
                        // Read from file
                        let mut total_bytes_written: u64 = 0;
                        loop {
                            // Read till you can
                            let mut buffer: [u8; 8192] = [0; 8192];
                            match rhnd.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    total_bytes_written += bytes_read as u64;
                                    if bytes_read == 0 {
                                        break;
                                    } else {
                                        // Write bytes
                                        if let Err(err) = lhnd.write(&buffer) {
                                            eprintln!("Could not write data to file: {}", err);
                                            return;
                                        }
                                        print_progress_bar(
                                            total_bytes_written as usize,
                                            file_size as usize,
                                            "Downloading file...",
                                        );
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Could not read file: {}", err);
                                    return;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Could not open local file '{}': {}", local.display(), err)
                    }
                }
            }
            Err(err) => eprintln!("Could not open remote file '{}': {}", remote.display(), err),
        }
    }

    /// ### ls
    ///
    /// List files in current directory
    pub fn ls(&self, argv: Vec<&str>) {
        let dir: PathBuf = match argv.get(1) {
            Some(d) => {
                let path = PathBuf::from(d);
                self.get_abs_path(path.as_path())
            }
            None => self.wrkdir.clone(),
        };
        match self.client.readdir(dir.as_path()) {
            Ok(files) => {
                println!(
                    "{:32}\t{:8}\t{:4}\t{:4}\t{:10}\t{:32}",
                    "Filename", "Size", "UID", "GID", "Mode", "Time"
                );
                for (path, metadata) in files {
                    // Format time
                    // Create a NaiveDateTime from the timestamp
                    let seconds: i64 = metadata.mtime.unwrap_or(0) as i64;
                    let naive = NaiveDateTime::from_timestamp(seconds, 0);
                    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                    // Format the datetime how you want
                    let newdate = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);
                    // let newdate = datetime.format("%Y-%m-%dT%H:%M:%S");
                    let file_name: PathBuf = PathBuf::from(path.as_path().file_name().unwrap());
                    // Print
                    println!(
                        "{:32}\t{:8}\t{:4}\t{:4}\t{:10}\t{:32}",
                        file_name.display(),
                        metadata.size.unwrap_or(0),
                        metadata.uid.unwrap_or(0),
                        metadata.gid.unwrap_or(0),
                        SftpClient::print_mode(metadata.perm.unwrap_or(0)),
                        newdate
                    );
                }
                println!("");
            }
            Err(err) => eprintln!("Could not get files in {}: {}", dir.display(), err),
        }
    }

    /// ### mkdir
    ///
    /// Make directory
    pub fn mkdir(&self, argv: Vec<&str>) {
        if argv.len() < 2 {
            eprintln!("Missing argument");
            self.usage();
            return;
        }
        let path = PathBuf::from(argv[1]);
        let path: PathBuf = self.get_abs_path(path.as_path());
        if let Err(err) = self.client.mkdir(path.as_path(), 0o755) {
            eprintln!("Could not create directory '{}': {}", path.display(), err);
        }
    }

    /// ### rmdir
    ///
    /// Remove directory
    pub fn rmdir(&self, argv: Vec<&str>) {
        if argv.len() < 2 {
            eprintln!("Missing argument");
            self.usage();
            return;
        }
        let path = PathBuf::from(argv[1]);
        let path: PathBuf = self.get_abs_path(path.as_path());
        if let Err(err) = self.client.rmdir(path.as_path()) {
            eprintln!("Could not remove directory '{}': {}", path.display(), err);
        }
    }

    pub fn mov(&self, argv: Vec<&str>) {
        if argv.len() < 3 {
            eprintln!("Missing argument");
            self.usage();
            return;
        }
        let src = PathBuf::from(argv[1]);
        let src: PathBuf = self.get_abs_path(src.as_path());
        let dst = PathBuf::from(argv[2]);
        let dst: PathBuf = self.get_abs_path(dst.as_path());
        if let Err(err) = self.client.rename(src.as_path(), dst.as_path(), None) {
            eprintln!(
                "Could not move '{}' to '{}': {}",
                src.display(),
                dst.display(),
                err
            );
        }
    }

    /// ### put
    ///
    /// Put file to remote
    pub fn put(&self, argv: Vec<&str>) {
        if argv.len() < 2 {
            eprintln!("Missing argument");
            self.usage();
            return;
        }
        let local = PathBuf::from(argv[1]);
        let local: PathBuf = self.get_abs_path(local.as_path());
        let remote: PathBuf = match argv.get(2) {
            Some(arg) => PathBuf::from(arg),
            None => {
                // Get absolute path from filename
                let mut p: PathBuf = self.wrkdir.clone();
                let file_name: PathBuf = PathBuf::from(local.as_path().file_name().unwrap());
                p.push(file_name);
                p
            }
        };
        // Open file on localhost
        match OpenOptions::new()
            .write(false)
            .read(true)
            .open(local.as_path())
        {
            Ok(mut lhnd) => {
                // Get file size
                let file_size: u64 = lhnd.seek(SeekFrom::End(0)).unwrap_or(0) as u64;
                // rewind
                if let Err(err) = lhnd.seek(SeekFrom::Start(0)) {
                    eprintln!("Could not rewind file: {}", err);
                    return;
                }
                // Open remote file
                match self.client.create(remote.as_path()) {
                    Ok(mut rhnd) => {
                        let mut total_bytes_written: u64 = 0;
                        loop {
                            // Read till you can
                            let mut buffer: [u8; 8192] = [0; 8192];
                            match lhnd.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    total_bytes_written += bytes_read as u64;
                                    if bytes_read == 0 {
                                        break;
                                    } else {
                                        // Write bytes
                                        if let Err(err) = rhnd.write(&buffer) {
                                            eprintln!("Could not write data to file: {}", err);
                                            return;
                                        }
                                        print_progress_bar(
                                            total_bytes_written as usize,
                                            file_size as usize,
                                            "Uploading file...",
                                        );
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Could not read file: {}", err);
                                    return;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Could not open remote file '{}': {}", remote.display(), err)
                    }
                }
            }
            Err(err) => eprintln!("Could not open file '{}': {}", local.display(), err),
        }
    }

    /// ### pwd
    ///
    /// Print working directory
    pub fn pwd(&self) {
        println!("{}", self.wrkdir.display());
    }

    /// ### usage
    ///
    /// Print commands
    pub fn usage(&self) {
        println!("CWD <dir>\t\tchange working directory");
        println!("DEL <file>\t\tremove file");
        println!("GET <file> [filename]\tdownload file from remote");
        println!("LS [dir]\t\tlist files in directory");
        println!("MKDIR <dir>\t\tmake a new directory");
        println!("MOV <src> <dst>\t\tMove file or directory");
        println!("RMDIR <dir>\t\tremove directory");
        println!("PUT <file> [filename]\tUpload file to current directory");
        println!("PWD\t\t\tPrint working directory");
        println!("QUIT\t\t\tquit client");
    }

    // Privates

    /// ### real_path
    ///
    /// Get real path from remote server
    fn real_path(&self, path: &Path) -> Option<PathBuf> {
        if let Ok(p) = self.client.realpath(path) {
            Some(p)
        } else {
            None
        }
    }

    /// ### get_abs_path
    ///
    /// Get absolute path from path argument
    fn get_abs_path(&self, p: &Path) -> PathBuf {
        match p.is_relative() {
            true => {
                let mut root: PathBuf = self.wrkdir.clone();
                root.push(p);
                match self.real_path(root.as_path()) {
                    Some(p) => p,
                    None => root,
                }
            }
            false => PathBuf::from(p),
        }
    }

    /// ### print_mode
    ///
    /// Print mode in LS format
    fn print_mode(mode: u32) -> String {
        let mut s: String = String::with_capacity(10);
        let file_type: u8 = ((mode >> 12) & 0x7) as u8;
        let user_pex: u8 = ((mode >> 6) & 0x7) as u8;
        let group_pex: u8 = ((mode >> 3) & 0x7) as u8;
        let others_pex: u8 = (mode & 0x7) as u8;
        s.push_str(match file_type {
            0 => "-",
            2 => "l",
            4 => "d",
            _ => {
                println!("Unknown mode {}", file_type);
                "?"
            }
        });
        let read: u8 = (user_pex >> 2) & 0x1;
        let write: u8 = (user_pex >> 1) & 0x1;
        let exec: u8 = user_pex & 0x1;
        s.push_str(match read {
            1 => "r",
            _ => "-",
        });
        s.push_str(match write {
            1 => "w",
            _ => "-",
        });
        s.push_str(match exec {
            1 => "x",
            _ => "-",
        });
        let read: u8 = (group_pex >> 2) & 0x1;
        let write: u8 = (group_pex >> 1) & 0x1;
        let exec: u8 = group_pex & 0x1;
        s.push_str(match read {
            1 => "r",
            _ => "-",
        });
        s.push_str(match write {
            1 => "w",
            _ => "-",
        });
        s.push_str(match exec {
            1 => "x",
            _ => "-",
        });
        let read: u8 = (others_pex >> 2) & 0x1;
        let write: u8 = (others_pex >> 1) & 0x1;
        let exec: u8 = others_pex & 0x1;
        s.push_str(match read {
            1 => "r",
            _ => "-",
        });
        s.push_str(match write {
            1 => "w",
            _ => "-",
        });
        s.push_str(match exec {
            1 => "x",
            _ => "-",
        });
        // return s
        s
    }
}

/// ### print_progress_bar
///
/// Print progress bar to stdout
fn print_progress_bar(it: usize, max: usize, prefix: &str) {
    let percentage: f64 = ((it as f64) * 100.0) / (max as f64);
    // Allocate bar
    let mut prog_bar: String = String::with_capacity(100);
    // For 100 times
    for i in 0..100 {
        if i <= percentage as i64 {
            prog_bar.push_str("█");
        } else {
            prog_bar.push_str(" ");
        }
    }
    // Print
    print!("\r{} [{}] {:.2}%", prefix, prog_bar, percentage);
    if it >= max {
        println!("");
    } else {
        // Flush
        io::stdout().flush().unwrap();
    }
}
