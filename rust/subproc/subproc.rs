//! ## Process
//!
//! `Process` contains the implementation for Sub process with pipes

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
extern crate tempfile;
extern crate uuid;

use super::pipe::Pipe;

use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// ### SubProcState
///
/// SubProcState represents the current subproc state
#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum SubProcState {
    Running,
    Terminated,
    Unknown
}

/// ### SubProcError
///
/// SubProcError represents an error caused by subproc module
#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum SubProcError {
    CouldNotStartProcess,
    InvalidData,
    IoTimeout,
    SubProcStillRunning,
    SubProcTerminated,
    CouldNotKill,
    PipeError(nix::errno::Errno)
}

/// ### ShellProc
/// 
/// Shell Proc represents an instance of the shell process wrapper
#[derive(std::fmt::Debug)]
pub struct ShellProc {
    pub state: SubProcState,                  //Shell process state
    pub pid: i32,                           //Subproc pid
    //Private
    rc: u8,                                 //Return code of the sub process
    stdout_cache: Option<String>,           //Used to prevent buffer fragmentation
    //Pipes
    stdin_pipe: Pipe,
    stdout_pipe: Pipe,
    stderr_pipe: Pipe
}

impl ShellProc {

    /// ### start
    /// 
    /// Start a process
    pub fn start(argv: Vec<String>) -> Result<ShellProc, SubProcError> {
        if argv.len() == 0 {
            return Err(SubProcError::CouldNotStartProcess)
        }
        //Create pipes
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let stdin_pipe: Pipe = match Pipe::open(&tmpdir.path().join("stdin.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        let stderr_pipe: Pipe = match Pipe::open(&tmpdir.path().join("stderr.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        let stdout_pipe: Pipe = match Pipe::open(&tmpdir.path().join("stdout.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        //Fork process
        match nix::unistd::fork() {
            Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
                //Return Shell Proc
                Ok(ShellProc {
                    state: SubProcState::Running,
                    pid: child.as_raw(),
                    rc: 255,
                    stdout_cache: None,
                    stdin_pipe: stdin_pipe,
                    stderr_pipe: stderr_pipe,
                    stdout_pipe: stdout_pipe
                })
            },
            Ok(nix::unistd::ForkResult::Child) => {
                std::process::exit(ShellProc::run(argv, stdin_pipe.fd, stderr_pipe.fd, stdout_pipe.fd));
            },
            Err(_) => {
                return Err(SubProcError::CouldNotStartProcess)
            }
        }
    }

    /// ### cleanup
    /// 
    /// cleanup subproc once exited. Returns the subrpoc exit code
    pub fn cleanup(&mut self) -> Result<u8, SubProcError> {
        if self.read_state() != SubProcState::Terminated {
            return Err(SubProcError::SubProcStillRunning)
        }
        //Close pipes
        let _ = self.stdin_pipe.close();
        let _ = self.stdout_pipe.close();
        let _ = self.stderr_pipe.close();
        Ok(self.rc)
    }

    /// ### raise
    /// 
    /// Send signal to shell
    pub fn raise(&self, signal: nix::sys::signal::Signal) -> Result<(), SubProcError> {
        match nix::sys::signal::kill(nix::unistd::Pid::from_raw(self.pid), signal) {
            Ok(_) => Ok(()),
            Err(_) => Err(SubProcError::CouldNotKill)
        }
    }

    /// ### kill
    /// 
    /// Kill shell sending SIGKILL
    pub fn kill(&self) -> Result<(), SubProcError> {
        self.raise(nix::sys::signal::Signal::SIGKILL)
    }
    
    /// ### read
    /// 
    /// Read from child pipes
    pub fn read(&mut self) -> Result<(Option<String>, Option<String>), SubProcError> {
        let stdout: Option<String> = match self.stdout_pipe.read(50, false) {
            Ok(stdout) => stdout,
            Err(err) => return Err(err)
        };
        let stderr: Option<String> = match self.stderr_pipe.read(50, false) {
            Ok(stderr) => match stderr {
                None => None,
                Some(stderr) => Some(stderr)
            },
            Err(err) => return Err(err)
        };
        Ok((stdout, stderr))
    }

    /// ### write
    /// 
    /// Write to child process stdin
    pub fn write(&mut self, mut data: String) -> Result<(), SubProcError> {
        if self.read_state() == SubProcState::Terminated {
            return Err(SubProcError::SubProcTerminated)
        }
        self.stdin_pipe.write(data, 5000)
    }

    /// ### run
    /// 
    /// Run method for thread
    fn run(argv: Vec<String>, stdin: RawFd, stderr: RawFd, stdout: RawFd) -> i32 {
        //Set child process stdout/stdin/stderr
        if let Err(_) = nix::unistd::dup2(stdin, 0) {
            return 255
        }
        if let Err(_) = nix::unistd::dup2(stdout, 1) {
            return 255
        }
        if let Err(_) = nix::unistd::dup2(stderr, 2) {
            return 255
        }
        //Prepare arguments
        let mut c_argv: Vec<CString> = Vec::with_capacity(argv.len());
        for arg in argv.iter() {
            c_argv.push(CString::new(arg.as_str()).unwrap());
        }
        let mut c_argv_refs: Vec<&CStr> = Vec::with_capacity(c_argv.len());
        for arg in c_argv.iter() {
            c_argv_refs.push(arg);
        }
        //Exec process
        if let Err(_) = nix::unistd::execvp(c_argv_refs.get(0).unwrap(), c_argv_refs.as_slice()) {
            return 255
        }
        return 0
    }

    /// ### read_state
    /// 
    /// Update subproc running state checking if the other thread has terminated
    pub fn read_state(&mut self) -> SubProcState {
        //Wait pid (NO HANG)
        match nix::sys::wait::waitpid(nix::unistd::Pid::from_raw(self.pid), Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
            Err(_) => {}, //Could not get information
            Ok(status) => match status {
                nix::sys::wait::WaitStatus::Exited(_, rc) => {
                    self.state = SubProcState::Terminated;
                    self.rc = rc as u8;
                },
                nix::sys::wait::WaitStatus::Signaled(_, signal, _) => {
                    self.state = SubProcState::Terminated;
                    self.rc = signal as u8;
                },
                _ => {}, //Still running
            }
        };
        self.state
    }

}

impl Drop for ShellProc {
    fn drop(&mut self) {
        if let Err(_) = self.cleanup() {
            let _ = self.kill(); //Force to terminate
            let _ = self.cleanup(); //Then finally clean up
        }
    }
}

//@! Test module

#[cfg(test)]
mod tests {

    use super::*;

    use nix::NixPath;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_process_start_stop() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("sh")]).unwrap();
        println!("A new subproc started with PID {}", shell_proc.pid);
        //Check shell parameters
        assert_eq!(shell_proc.state, SubProcState::Running);
        assert_ne!(shell_proc.pid, 0);
        assert_eq!(shell_proc.rc, 255);
        assert!(shell_proc.stdout_cache.is_none());
        //Verify shell is still running
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.read_state(), SubProcState::Running);
        //Stop process
        assert!(shell_proc.kill().is_ok());
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.read_state(), SubProcState::Terminated);
        //Rc should be set to 9
        assert_eq!(shell_proc.state, SubProcState::Terminated);
        assert_eq!(shell_proc.rc, 9);
        //Cleanup
        assert!(shell_proc.cleanup().is_ok());
    }

    #[test]
    fn test_process_start_error() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("piroporopero")]).unwrap();
        println!("A new subproc started with PID {}", shell_proc.pid);
        //Shell should have died
        sleep(Duration::from_millis(1000));
        assert_eq!(shell_proc.read_state(), SubProcState::Terminated);
        assert_eq!(shell_proc.rc, 255);
    }

    #[test]
    fn test_process_raise() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("sh")]).unwrap();
        println!("A new subproc started with PID {}", shell_proc.pid);
        //Verify shell is still running
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.read_state(), SubProcState::Running);
        //Send SIGINT
        assert!(shell_proc.raise(nix::sys::signal::Signal::SIGINT).is_ok());
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.read_state(), SubProcState::Terminated);
        assert_eq!(shell_proc.rc, 2);
    }

}
