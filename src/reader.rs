use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type LineResult = Result<(usize, String), std::io::Error>;

pub struct LogReader {
    reader: Box<dyn BufRead>,
}

impl LogReader {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path_ref = path.as_ref();
        let file = File::open(path_ref)?;

        let is_gz = path_ref
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("gz"))
            .unwrap_or(false);

        let reader: Box<dyn BufRead> = if is_gz {
            let decoder = GzDecoder::new(file);
            Box::new(BufReader::with_capacity(8 * 1024, decoder))
        } else {
            Box::new(BufReader::with_capacity(8 * 1024, file))
        };

        Ok(Self { reader })
    }

    pub fn lines(&mut self) -> LineIterator<'_> {
        LineIterator {
            reader: &mut *self.reader,
            line_num: 0,
        }
    }
}

pub struct LineIterator<'a> {
    reader: &'a mut dyn BufRead,
    line_num: usize,
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = LineResult;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();

        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => {
                self.line_num += 1;

                if line.ends_with('\n') {
                    line.pop();
                }
                if line.ends_with('\r') {
                    line.pop();
                }
                Some(Ok((self.line_num, line)))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::io::Write;
    use tempfile::{NamedTempFile, tempdir};

    #[test]
    fn test_reader_lines_plain_files() {
        let mut temp_file = NamedTempFile::new().expect("create temp file");
        writeln!(temp_file, "line 1").expect("write line 1");
        writeln!(temp_file, "line 2").expect("write line 2");
        writeln!(temp_file, "line 3").expect("write line 3");

        let mut reader = LogReader::new(temp_file.path()).expect("create reader");
        let lines: Vec<_> = reader.lines().map(|l| l.expect("line read")).collect();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], (1, "line 1".to_string()));
        assert_eq!(lines[1], (2, "line 2".to_string()));
        assert_eq!(lines[2], (3, "line 3".to_string()));
    }

    #[test]
    fn test_reader_lines_gz_file() {
        let dir = tempdir().expect("create temp dir");
        let gz_path = dir.path().join("sample.log.gz");

        let gz_file = File::create(&gz_path).expect("create gz file");
        let mut encoder = GzEncoder::new(gz_file, Compression::default());

        writeln!(encoder, "line 1").expect("write line 1");
        writeln!(encoder, "line 2").expect("write line 2");
        writeln!(encoder, "line 3").expect("write line 3");
        encoder.finish().expect("finish encoding");

        let mut reader = LogReader::new(&gz_path).expect("create reader");
        let lines: Vec<_> = reader.lines().map(|l| l.expect("line read")).collect();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], (1, "line 1".to_string()));
        assert_eq!(lines[1], (2, "line 2".to_string()));
        assert_eq!(lines[2], (3, "line 3".to_string()));
    }
}
