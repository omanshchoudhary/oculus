use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type LineResult = Result<(usize, String), std::io::Error>;

pub struct LogReader {
    reader: BufReader<File>,
}

impl LogReader {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        // Get a file handle to the disk
        let file = File::open(path)?;
        // Add buffere ability
        let reader = BufReader::with_capacity(8 * 1024, file);
        Ok(Self { reader })
    }

    pub fn lines(&mut self) -> LineIterator<'_> {
        LineIterator {
            reader: &mut self.reader,
            line_num: 0,
        }
    }
}

pub struct LineIterator<'a> {
    reader: &'a mut BufReader<File>,
    line_num: usize,
}
impl<'a> Iterator for LineIterator<'a> {
    type Item = LineResult;

    fn next(&mut self) -> Option<Self::Item> {
        // Tracking line number
        self.line_num += 1;

        // Reading a line and storing it into a string
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None, // EOF
            Ok(_) => {
                // Remove trailing whitespace or new line
                let _ = line.pop();
                if let Some('\r') = line.chars().last() {
                    let _ = line.pop();
                }
                Some(Ok((self.line_num, line)))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    #[test]
    fn test_reader_lines() {
        // Create a temp file with test data
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "line 1").unwrap();
        writeln!(temp_file, "line 2").unwrap();
        writeln!(temp_file, "line 3").unwrap();
        let mut reader = LogReader::new(temp_file.path()).unwrap();
        
        let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();
        
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], (1, "line 1".to_string()));
        assert_eq!(lines[1], (2, "line 2".to_string()));
        assert_eq!(lines[2], (3, "line 3".to_string()));
    }
}