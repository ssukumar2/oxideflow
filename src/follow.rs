use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::Duration;
use crate::parser;
use crate::output;

pub fn tail_file(path: &str, colored: bool) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Seek to end
    reader.seek(SeekFrom::End(0))?;
    let mut line_number = 0usize;

    // Count existing lines for line_number offset
    {
        let count_file = File::open(path)?;
        let count_reader = BufReader::new(count_file);
        line_number = count_reader.lines().count();
    }

    println!("--- following {} (Ctrl+C to stop) ---", path);

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;

        if bytes == 0 {
            thread::sleep(Duration::from_millis(200));
            continue;
        }

        line_number += 1;
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            continue;
        }

        let parsed = parser::parse_line(trimmed, line_number);
        if colored {
            output::print_colored(&parsed);
        } else {
            output::print_plain(&parsed);
        }
    }
}
