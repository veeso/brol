//! ## File
//!
//! `File` module implements some utilities related to files

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::Path;

/// ### read_lines
/// 
/// Read lines from file
pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    let reader = io::BufReader::new(file).lines();
    let mut lines: Vec<String> = Vec::new();
    for line in reader {
        if let Ok(line) = line {
            lines.push(line);
        }
    }
    Ok(lines)
}

/// ### write_lines
/// 
/// Write lines to file
pub fn write_lines<P>(filename: P, lines: Vec<String>) -> io::Result<()> where P: AsRef<Path> {
    match OpenOptions::new().create(true).write(true).append(false).truncate(true).open(filename) {
        Ok(mut f) => {
            for line in lines.iter() {
                writeln!(f, "{}", line)?;
            }
            Ok(())
        },
        Err(err) => Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_utils_file_read_lines() {
        let sample_file: tempfile::NamedTempFile = write_sample_file();
        let res: io::Result<Vec<String>> = read_lines(sample_file.path());
        assert!(res.is_ok());
        let lines: Vec<String> = res.unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(*lines.get(0).unwrap(), String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."));
        assert_eq!(*lines.get(1).unwrap(), String::from("Mauris ultricies consequat eros,"));
        assert_eq!(*lines.get(2).unwrap(), String::from("nec scelerisque magna imperdiet metus."));
    }

    #[test]
    fn test_utils_file_read_lines_no_file() {
        assert!(read_lines(Path::new("/sample.file123123.txt")).is_err());
    }

    #[test]
    fn test_utils_file_write_lines() {
        let in_lines: Vec<String> = vec![String::from("row 1"), String::from("row 2"), String::from("row 3")];
        let tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        assert!(write_lines(tmpfile.path(), in_lines.clone()).is_ok());
        //Verify rows
        let res: io::Result<Vec<String>> = read_lines(tmpfile.path());
        assert!(res.is_ok());
        let out_lines: Vec<String> = res.unwrap();
        assert_eq!(in_lines, out_lines);
    }

    #[test]
    fn test_utils_file_write_lines_error() {
        let in_lines: Vec<String> = vec![String::from("row 1"), String::from("row 2"), String::from("row 3")];
        assert!(write_lines(Path::new("/sample.file1231234.txt"), in_lines).is_err());
    }

    /// ### write_sample_file
    /// Write a sample file
    fn write_sample_file() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nMauris ultricies consequat eros,\nnec scelerisque magna imperdiet metus.\n"
        )
        .unwrap();
        tmpfile
    }
}
