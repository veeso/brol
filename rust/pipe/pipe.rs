//! ## Pipe
//!
//! `Pipe` provides an api to interface with UNIX pipes

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "Pyc"
*
*   Pyc is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   Pyc is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with Pyc.  If not, see <http://www.gnu.org/licenses/>.
*
*/

extern crate nix;

use super::{ShellError};

use std::path::PathBuf;
use std::os::unix::io::RawFd;
use std::time::{Instant, Duration};

//UNIX
use nix::unistd;

#[derive(Clone, std::fmt::Debug)]
pub(crate) struct Pipe {
    pub path: PathBuf, //Pipe path
    pub fd: RawFd
}

impl Pipe {

    /// ### open
    /// 
    /// Open and creates a new pipe. Returns pipe on suceess or shell error
    pub fn open(path: &PathBuf) -> Result<Pipe, ShellError> {
        //Mkfifo - Not necessary with O_CREAT
        if let Err(err) = unistd::mkfifo(path.as_path(), nix::sys::stat::Mode::S_IRWXU | nix::sys::stat::Mode::S_IRWXG | nix::sys::stat::Mode::S_IRWXO) {
            match err {
                nix::Error::Sys(errno) => return Err(ShellError::PipeError(errno)),
                _ => return Err(ShellError::PipeError(nix::errno::Errno::UnknownErrno))
            }
        }
        //Open fifo
        match nix::fcntl::open(path.as_path(), nix::fcntl::OFlag::O_RDWR, nix::sys::stat::Mode::S_IRWXU | nix::sys::stat::Mode::S_IRWXG | nix::sys::stat::Mode::S_IRWXO) {
            Ok(fd) => {
                Ok(Pipe {
                    path: path.clone(),
                    fd: fd
                })
            },
            Err(err) => {
                match err {
                    nix::Error::Sys(errno) => Err(ShellError::PipeError(errno)),
                    _ => Err(ShellError::PipeError(nix::errno::Errno::UnknownErrno))
                }
            }
        }
    }

    /// ### close
    /// 
    /// Close and delete pipe
    pub fn close(&self) -> Result<(), ShellError> {
        if let Err(err) = unistd::close(self.fd) {
            match err {
                nix::Error::Sys(errno) => return Err(ShellError::PipeError(errno)),
                _ => return Err(ShellError::PipeError(nix::errno::Errno::UnknownErrno))
            }
        };
        //Unlink pipe
        let _ = unistd::unlink(self.path.as_path());
        Ok(())
    }

    /// ### read
    /// 
    /// Read from pipe
    /// If read_all parameter is False, then the function returns after reading 8192 or less
    /// otherwise, if set to True, reads until there's something available to be read
    pub fn read(&self, timeout: u64, read_all: bool) -> Result<Option<String>, ShellError> {
        //Create poll fd wrapper
        let mut poll_fds: [nix::poll::PollFd; 1] = [nix::poll::PollFd::new(self.fd, nix::poll::PollFlags::POLLIN | nix::poll::PollFlags::POLLRDBAND | nix::poll::PollFlags::POLLHUP)];
        //Prepare out buffer
        let mut data_out: String = String::new();
        let mut data_size: usize = 0;
        //Prepare times
        let timeout: Duration = Duration::from_millis(timeout);
        let time: Instant = Instant::now();
        while time.elapsed() < timeout {
            //Poll pipe
            match nix::poll::poll(&mut poll_fds, 50) {
                Ok(ret) => {
                    if ret > 0 && poll_fds[0].revents().is_some() { //Fifo is available to be read
                        let event: nix::poll::PollFlags = poll_fds[0].revents().unwrap();
                        if event.intersects(nix::poll::PollFlags::POLLIN) || event.intersects(nix::poll::PollFlags::POLLRDBAND) {
                            //Read from FIFO
                            let mut buffer: [u8; 8192] = [0; 8192];
                            match unistd::read(self.fd, &mut buffer) {
                                Ok(bytes_read) => {
                                    data_size += bytes_read;
                                    //Push bytes converted to string to data out
                                    data_out.push_str(match std::str::from_utf8(&buffer[0..bytes_read]) {
                                        Ok(s) => s,
                                        Err(_) => {
                                            return Err(ShellError::InvalidData)
                                        }
                                    });
                                    if ! read_all {
                                        break;
                                    }
                                },
                                Err(err) => {
                                    match err {
                                        nix::Error::Sys(errno) => {
                                            match errno {
                                                nix::errno::Errno::EAGAIN => { //No more data is available to be read
                                                    if data_size == 0 {
                                                        continue; //Keep waiting for data
                                                    } else {
                                                        break; //All data has been read
                                                    }
                                                },
                                                _ => return Err(ShellError::PipeError(errno)) //Error
                                            }
                                        },
                                        _ => return Err(ShellError::PipeError(nix::errno::Errno::UnknownErrno))
                                    }
                                }
                            };
                        } else if event.intersects(nix::poll::PollFlags::POLLERR) { //FIFO is in error state
                            return Err(ShellError::PipeError(nix::errno::Errno::EPIPE))
                        } else if event.intersects(nix::poll::PollFlags::POLLHUP) { //No more data
                            //no data is available; if data is something break; otherwise continue
                            if data_size == 0 {
                                continue;
                            } else {
                                break;
                            }
                        }
                    } else if ret == 0 {
                        //no data is available; if data is something break; otherwise continue
                        if data_size == 0 {
                            continue;
                        } else {
                            break;
                        }
                    }
                },
                Err(err) => { //Handle poll error
                    match err {
                        nix::Error::Sys(errno) => {
                            match errno {
                                nix::errno::Errno::EAGAIN => { //No more data is available to be read
                                    if data_size == 0 {
                                        continue; //Keep waiting for data
                                    } else {
                                        break; //All data has been read
                                    }
                                },
                                _ => return Err(ShellError::PipeError(errno)) //Error
                            }
                        },
                        _ => return Err(ShellError::PipeError(nix::errno::Errno::UnknownErrno))
                    }
                }
            }
        }
        //Return data
        match data_size {
            0 => Ok(None),
            _ => Ok(Some(data_out))
        }
    }

    /// ### write
    /// 
    /// Write data out to pipe
    pub fn write(&self, data: String, timeout: u64) -> Result<(), ShellError> {
        //Create poll fd wrapper
        let mut poll_fds: [nix::poll::PollFd; 1] = [nix::poll::PollFd::new(self.fd, nix::poll::PollFlags::POLLOUT)];
        //Prepare times
        let timeout: Duration = Duration::from_millis(timeout);
        let time: Instant = Instant::now();
        //Prepare data out
        let data_out = data.as_bytes();
        let total_bytes_amount: usize = data_out.len();
        //Write bytes
        let mut bytes_written: usize = 0;
        while bytes_written < total_bytes_amount {
            match nix::poll::poll(&mut poll_fds, 50) {
                Ok(_) => {
                    if let Some(revents) = poll_fds[0].revents() {
                        if revents.intersects(nix::poll::PollFlags::POLLOUT) {
                            //Write data out (8192 or remaining bytes)
                            let bytes_out = if total_bytes_amount - bytes_written > 8192 {
                                8192
                            } else {
                                total_bytes_amount - bytes_written
                            };
                            //Write data out
                            match unistd::write(self.fd, &data_out[bytes_written..(bytes_written + bytes_out)]) {
                                Ok(bytes) => {
                                    bytes_written += bytes; //Increment bytes written of bytes
                                },
                                Err(err) => {
                                    match err {
                                        nix::Error::Sys(errno) => return Err(ShellError::PipeError(errno)),
                                        _ => return Err(ShellError::PipeError(nix::errno::Errno::UnknownErrno))
                                    }
                                }
                            }
                        }
                    }
                },
                Err(err) => {
                    match err {
                        nix::Error::Sys(errno) => return Err(ShellError::PipeError(errno)),
                        _ => return Err(ShellError::PipeError(nix::errno::Errno::UnknownErrno))
                    }
                }
            };
            if bytes_written == 0 && time.elapsed() >= timeout {
                //Return Io Timeout
                return Err(ShellError::IoTimeout);
            }
        }
        Ok(())
    }

}

//@! Test module

#[cfg(test)]
mod tests {

    use super::*;

    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_pipe_open_close() {
        let tmpdir: tempfile::TempDir = create_tmp_dir();
        let pipe_path: PathBuf = tmpdir.path().join("test.fifo");
        let pipe: Result<Pipe, ShellError> = Pipe::open(&pipe_path);
        assert!(pipe.is_ok(), format!("Pipe ({}) should be OK, but is {:?}", pipe_path.display(), pipe));
        let pipe: Pipe = pipe.unwrap();
        assert_eq!(pipe.path, pipe_path);
        assert!(pipe.fd > 0);
        assert!(pipe.close().is_ok());
    }

    #[test]
    fn test_pipe_io() {
        let tmpdir: tempfile::TempDir = create_tmp_dir();
        let pipe_path: PathBuf = tmpdir.path().join("stdout.fifo");
        //Open Pipe
        let pipe: Result<Pipe, ShellError> = Pipe::open(&pipe_path);
        assert!(pipe.is_ok(), format!("Pipe ({}) should be OK, but is {:?}", pipe_path.display(), pipe));
        let pipe: Pipe = pipe.unwrap();
        let pipe_thread: Pipe = pipe.clone();
        //Start thread
        let join_hnd: thread::JoinHandle<()> = thread::spawn(move || {
            let input: String = pipe_thread.read(1000, true).unwrap().unwrap();
            assert_eq!(input, String::from("HELLO\n"));
            thread::sleep(Duration::from_millis(100)); //Sleep for 100 msecond
            //Write
            assert!(pipe_thread.write(String::from("HI THERE\n"), 1000).is_ok());
        });
        //Write pipe
        assert!(pipe.write(String::from("HELLO\n"), 1000).is_ok(), "Write timeout");
        //Read pipe
        thread::sleep(Duration::from_millis(100)); //Sleep for 100 msecond
        let read: Result<Option<String>, ShellError> = pipe.read(1000, true);
        assert!(read.is_ok(), format!("Read should be Ok, but is {:?}", read));
        let read: Option<String> = read.unwrap();
        assert_eq!(read.unwrap(), String::from("HI THERE\n"));
        //Join thread
        assert!(join_hnd.join().is_ok());
        //Close Pipe
        assert!(pipe.close().is_ok());
    }

    #[test]
    fn test_pipe_read_all() {
        let tmpdir: tempfile::TempDir = create_tmp_dir();
        let pipe_path: PathBuf = tmpdir.path().join("stdout.fifo");
        //Open Pipe
        let pipe: Result<Pipe, ShellError> = Pipe::open(&pipe_path);
        assert!(pipe.is_ok(), format!("Pipe ({}) should be OK, but is {:?}", pipe_path.display(), pipe));
        let pipe: Pipe = pipe.unwrap();
        let pipe_thread: Pipe = pipe.clone();
        //Start thread
        let join_hnd: thread::JoinHandle<()> = thread::spawn(move || {
            let mut data: String = String::with_capacity(10240);
            for _ in 0..10240 {
                data.push('c');
            }
            //Write 10240 bytes
            assert!(pipe_thread.write(data.clone(), 1000).is_ok());
            thread::sleep(Duration::from_millis(500)); //Sleep for 500 msecond
            //Write
            assert!(pipe_thread.write(data, 1000).is_ok());
        });
        //Read all (10240 bytes should be read)
        assert_eq!(pipe.read(500, true).unwrap().unwrap().len(), 10240);
        //Read all set to false
        thread::sleep(Duration::from_millis(500)); //Sleep for 500 msecond
        //Now only 8192 bytes should have been read
        assert_eq!(pipe.read(500, false).unwrap().unwrap().len(), 8192);
        //Now finish to read
        assert_eq!(pipe.read(500, false).unwrap().unwrap().len(), 2048);
        //Join thread
        assert!(join_hnd.join().is_ok());
        //Close Pipe
        assert!(pipe.close().is_ok());
    }

    #[test]
    fn test_pipe_open_close_error() {
        //Open error
        let pipe_path: PathBuf = PathBuf::from("/dev/tty1");
        let pipe: Result<Pipe, ShellError> = Pipe::open(&pipe_path);
        assert!(pipe.is_err());
        //Close error
        let pipe: Pipe = Pipe {
            fd: 10,
            path: PathBuf::from("/tmp/stdout.fifo")
        };
        assert!(pipe.close().is_err());
    }

    #[test]
    fn test_pipe_io_error() {
        let tmpdir: tempfile::TempDir = create_tmp_dir();
        let pipe_path: PathBuf = tmpdir.path().join("stdout.fifo");
        //Open Pipe
        let pipe: Result<Pipe, ShellError> = Pipe::open(&pipe_path);
        assert!(pipe.is_ok(), format!("Pipe ({}) should be OK, but is {:?}", pipe_path.display(), pipe));
        let pipe: Pipe = pipe.unwrap();
        //assert!(pipe.write(String::from("HELLO\n"), 1000).is_err(), "Write should time out");
        assert!(pipe.read(1000, true).unwrap().is_none(), "Read should be None");
        assert!(pipe.close().is_ok());
    }

    fn create_tmp_dir() -> tempfile::TempDir {
        tempfile::TempDir::new().unwrap()
    }

}
