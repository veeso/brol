//! ## Console
//!
//! `Console` module provides an API for the terminal console

extern crate nix;
extern crate termios;

use std::io::{self, Read, Write};
use std::os::unix::io::RawFd;

const STDIN_FILENO: RawFd = 0;

/// ## InputEvent
/// 
/// InputEvent enum represents an Input Event got from user on a read call
#[derive(std::fmt::Debug, std::cmp::PartialEq)]
pub enum InputEvent {
    Key(String),
    Ctrl(u8),
    Enter,
    CarriageReturn,
    Backspace,
    ArrowUp,
    ArrowLeft,
    ArrowRight,
    ArrowDown
}


/// ### backspace
/// 
/// Remove last typed character from prompt
pub fn backspace() {
    //To backspace we have to go back of 1 position, print blank and go back again
    print(String::from("\x08 \x08"));
}

pub fn move_cursor_right() {
    print(String::from("\x1b[1C"));
}

pub fn move_cursor_left() {
    print(String::from("\x1b[1D"));
}

/// ### carriage_return
/// 
/// Return to the beginning of the line
pub fn carriage_return() {
    print(String::from("\r"));
}

/// ### clear
/// 
/// Clear console
pub fn clear() {
    print(String::from("\x1b[H\x1b[2J"));
}

/// ### read
/// 
/// Read user input and returns an individual InputEvent (or None)
pub fn read() -> Option<InputEvent> {
    let stdin_read = |buff: &mut [u8]| -> io::Result<()> {
        io::stdin().read_exact(buff)
    };
    prepare_termios();
    let ev: Option<InputEvent> = to_input_event(&input_ready, &stdin_read);
    reset_termios();
    ev
}

/// ### to_input_event
/// 
/// Get input through callback and convert it to an Input Event
fn to_input_event(ready_fn: &dyn Fn() -> bool, read_fn: &dyn Fn(&mut [u8]) -> io::Result<()>) -> Option<InputEvent> {
    //Configure terminal
    match ready_fn() {
        false => None,
        true => {
            //Read
            let mut buf: Vec<u8> = vec![0u8; 1];
            let _ = read_fn(&mut buf);
            //Handle input
            let key: u8 = *buf.get(0).unwrap_or(&0);
            let ev: InputEvent = match key {
                8 | 127 => InputEvent::Backspace,
                10 => InputEvent::Enter,
                13 => InputEvent::CarriageReturn,
                0..=26 => InputEvent::Ctrl(key), //CTRL key (exclude 8, 10, 13)
                27 => { //Is Arrow Key
                    //Read twice
                    let _ = read_fn(&mut buf);
                    let _ = read_fn(&mut buf);
                    let direction: char = *buf.get(0).unwrap_or(&0) as char;
                    match direction {
                        'A' => InputEvent::ArrowUp,
                        'B' => InputEvent::ArrowDown,
                        'C' => InputEvent::ArrowRight,
                        'D' => InputEvent::ArrowLeft,
                        _ => return None //Unknown event
                    }
                },
                _ => { //Handle normal key
                    //@! Read until it's a valid UTF8 string
                    //NOTE: 4 is the maximum amount of bytes used by a UTF-8
                    let mut utfbuffer: [u8; 4] = [0; 4];
                    let mut buff_index: usize = 0;
                    let mut keystr: Option<String> = None;
                    loop {
                        //Copy last character into utf buffer
                        if buff_index >= 4 { //Overflow
                            break
                        }
                        utfbuffer[buff_index] = *buf.get(0).unwrap_or(&0);
                        buff_index += 1;
                        //Check if utf buffer is a valid utf8 string
                        match std::str::from_utf8(&utfbuffer[0..buff_index]) { //If buffer is a valid
                            Ok(key) => {
                                keystr = Some(String::from(key));
                                break
                            },
                            Err(_) => { //If not valid...
                                if let Err(_) = read_fn(&mut buf) {
                                    break
                                }
                                continue
                            }
                        };
                    }
                    match keystr {
                        Some(s) => InputEvent::Key(s),
                        None => return None //Unknown key
                    }
                }
            };
            Some(ev)
        }
    }
}

/// ### rewrite
/// 
/// Rewrite current stdout line
pub fn rewrite(row: String, len: usize) {
    for _ in 0..len {
        backspace();
    }
    print(row);
}

/// ### print
/// 
/// print on this line without newline
pub fn print(row: String) {
    print!("{}", row);
    let _ = io::stdout().flush();
}

/// ### println
/// 
/// Print line and go to new line
pub fn println(row: String) {
    println!("{}", row);
}

/// ### input_ready
/// 
/// Returns whether stdin is ready to be read
fn input_ready() -> bool {
    prepare_termios();
    let mut poll_fds: [nix::poll::PollFd; 1] = [nix::poll::PollFd::new(STDIN_FILENO, nix::poll::PollFlags::POLLIN | nix::poll::PollFlags::POLLRDBAND | nix::poll::PollFlags::POLLHUP)];
    let ready: bool = match nix::poll::poll(&mut poll_fds, 100) {
        Ok(ret) => {
            if ret > 0 && poll_fds[0].revents().is_some() { //Stdin is available to be read
                let event: nix::poll::PollFlags = poll_fds[0].revents().unwrap();
                if event.intersects(nix::poll::PollFlags::POLLIN) || event.intersects(nix::poll::PollFlags::POLLRDBAND) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        },
        Err(_) => false
    };
    reset_termios();
    ready
}

/// ### prepare_termios
/// 
/// Prepare termios for console
fn prepare_termios() {
    let mut term = termios::Termios::from_fd(STDIN_FILENO).unwrap();
    let _ = termios::tcgetattr(STDIN_FILENO, &mut term);
    term.c_lflag &= !termios::ICANON;
    term.c_lflag &= !termios::ECHO;
    let _ = termios::tcsetattr(STDIN_FILENO, termios::TCSANOW, &term);
}

/// ### reset_termios
/// 
/// Restore previous termios configuration
fn reset_termios() {
    let mut term = termios::Termios::from_fd(STDIN_FILENO).unwrap();
    let _ = termios::tcgetattr(STDIN_FILENO, &mut term);
    term.c_lflag |= termios::ICANON;
    term.c_lflag &= termios::ECHO;
    let _ = termios::tcsetattr(STDIN_FILENO, termios::TCSADRAIN, &term);
}

/// ### input_event_to_string
/// 
/// Converts an input event to a string
pub fn input_event_to_string(ev: InputEvent) -> String {
    match ev {
        InputEvent::ArrowDown => String::from("\x1b[B"),
        InputEvent::ArrowLeft => String::from("\x1b[D"),
        InputEvent::ArrowRight => String::from("\x1b[C"),
        InputEvent::ArrowUp => String::from("\x1b[A"),
        InputEvent::Backspace => String::from("\x7F"),
        InputEvent::CarriageReturn => String::from("\x0D"),
        InputEvent::Ctrl(sig) => {
            let ch = sig as char;
            let mut s = String::new();
            s.push(ch);
            s
        },
        InputEvent::Enter => String::from("\x0A"),
        InputEvent::Key(k) => String::from(k)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_utils_console_backspace() {
        backspace();
    }

    #[test]
    fn test_utils_console_move_cursor() {
        move_cursor_left();
        move_cursor_right();
        carriage_return();
    }

    #[test]
    fn test_utils_console_clear() {
        clear();
    }

    #[test]
    fn test_utils_console_print() {
        print(String::from("foo"));
        rewrite(String::from("Foobar"), 3);
        println(String::from("bar"));
    }

    #[test]
    fn test_utils_console_input_ready() {
        assert_eq!(input_ready(), false);
    }
    
    #[test]
    fn test_utils_console_termios() {
        prepare_termios();
        reset_termios();
    }

    #[test]
    fn test_utils_console_read() {
        assert!(read().is_none());
        //Test read - input ready false
        let ready_fn = || -> bool {
            false
        };
        let read_fn = |_buff: &mut [u8]| -> io::Result<()> {
            Ok(())
        };
        assert!(to_input_event(&ready_fn, &read_fn).is_none());
        //Teast read - Backspace
        let ready_fn = || -> bool {
            true
        };
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            buff[0] = 127;
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::Backspace);
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            buff[0] = 8;
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::Backspace);
        //Test read - enter
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            buff[0] = 10;
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::Enter);
        //Test read - Carriage return
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            buff[0] = 13;
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::CarriageReturn);
        //Test read - Ctrl key
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            buff[0] = 3;
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::Ctrl(3));
        //Test read - Arrow key
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            let curr_value: u8 = buff[0];
            match curr_value {
                91 => buff[0] = 'A' as u8,
                27 => buff[0] = 91,
                _ => buff[0] = 27
            }
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::ArrowUp);
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            let curr_value: u8 = buff[0];
            match curr_value {
                91 => buff[0] = 'B' as u8,
                27 => buff[0] = 91,
                _ => buff[0] = 27
            }
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::ArrowDown);
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            let curr_value: u8 = buff[0];
            match curr_value {
                91 => buff[0] = 'C' as u8,
                27 => buff[0] = 91,
                _ => buff[0] = 27
            }
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::ArrowRight);
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            let curr_value: u8 = buff[0];
            match curr_value {
                91 => buff[0] = 'D' as u8,
                27 => buff[0] = 91,
                _ => buff[0] = 27
            }
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::ArrowLeft);
        //Unknown Arrow
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            let curr_value: u8 = buff[0];
            match curr_value {
                91 => buff[0] = 'E' as u8,
                27 => buff[0] = 91,
                _ => buff[0] = 27
            }
            Ok(())
        };
        assert!(to_input_event(&ready_fn, &read_fn).is_none());
        //Test read - ASCII key
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            buff[0] = 'A' as u8;
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::Key(String::from("A")));
        //Test read - UTF8 (п)
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            let curr_value: u8 = buff[0];
            match curr_value {
                0xd0 => buff[0] = 0xbf,
                _ => buff[0] = 0xd0
            }
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::Key(String::from("п")));
        //Test read - UTF8 (😂)
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            let curr_value: u8 = buff[0];
            match curr_value {
                0x98 => buff[0] = 0x82,
                0x9f => buff[0] = 0x98,
                0xf0 => buff[0] = 0x9f,
                _ => buff[0] = 0xf0
            }
            Ok(())
        };
        assert_eq!(to_input_event(&ready_fn, &read_fn).unwrap(), InputEvent::Key(String::from("😂")));
        //Unknown key
        let read_fn = |buff: &mut [u8]| -> io::Result<()> {
            let curr_value: u8 = buff[0];
            match curr_value {
                0x98 => buff[0] = 0xff,
                0x9f => buff[0] = 0x98,
                0xf0 => buff[0] = 0x9f,
                _ => buff[0] = 0xf0
            }
            Ok(())
        };
        assert!(to_input_event(&ready_fn, &read_fn).is_none());
    }

    #[test]
    fn test_utils_console_input_event_to_str() {
        assert_eq!(input_event_to_string(InputEvent::ArrowDown), String::from("\x1b[B"));
        assert_eq!(input_event_to_string(InputEvent::ArrowLeft), String::from("\x1b[D"));
        assert_eq!(input_event_to_string(InputEvent::ArrowRight), String::from("\x1b[C"));
        assert_eq!(input_event_to_string(InputEvent::ArrowUp), String::from("\x1b[A"));
        assert_eq!(input_event_to_string(InputEvent::Backspace), String::from("\x7F"));
        assert_eq!(input_event_to_string(InputEvent::CarriageReturn), String::from("\x0D"));
        assert_eq!(input_event_to_string(InputEvent::Ctrl(3)), String::from("\x03"));
        assert_eq!(input_event_to_string(InputEvent::Enter), String::from("\x0A"));
        assert_eq!(input_event_to_string(InputEvent::Key(String::from("A"))), String::from("A"));
    }

}
