use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

use std::fs::OpenOptions;
#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

/// Tailer for reading new lines appended to a log file
pub struct LogTailer {
    reader: BufReader<File>,
    position: u64,
}

impl LogTailer {
    /// Create a new LogTailer for the specified file path
    /// On Windows, opens the file with shared read/write access to avoid blocking Traefik
    /// Initially seeks to the end of the file to ignore existing entries
    pub fn new(path: &str) -> Result<Self> {
        // Open file with shared read/write access on Windows
        #[cfg(windows)]
        let file = OpenOptions::new()
            .read(true)
            .share_mode(0x01 | 0x02) // FILE_SHARE_READ | FILE_SHARE_WRITE
            .open(path)
            .context(format!("Failed to open file: {}", path))?;

        // For non-Windows platforms (for testing)
        #[cfg(not(windows))]
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .context(format!("Failed to open file: {}", path))?;

        let mut reader = BufReader::new(file);

        // Seek to end of file to ignore existing entries
        let position = reader
            .seek(SeekFrom::End(0))
            .context("Failed to seek to end of file")?;

        Ok(LogTailer { reader, position })
    }

    /// Read new lines that have been appended to the file since the last read
    /// Returns a vector of new complete lines
    pub fn read_new_lines(&mut self) -> Result<Vec<String>> {
        let mut lines = Vec::new();

        // Check if there's new data
        let file_size = self
            .reader
            .seek(SeekFrom::End(0))
            .context("Failed to seek to end of file")?;

        if file_size <= self.position {
            // No new data, restore position and return empty
            self.reader
                .seek(SeekFrom::Start(self.position))
                .context("Failed to restore file position")?;
            return Ok(lines);
        }

        // Seek to our last read position
        self.reader
            .seek(SeekFrom::Start(self.position))
            .context("Failed to seek to last position")?;

        // Read new lines
        loop {
            let mut line = String::new();
            let bytes_read = self
                .reader
                .read_line(&mut line)
                .context("Failed to read line from file")?;

            if bytes_read == 0 {
                // End of file reached
                break;
            }

            // Update position
            self.position += bytes_read as u64;

            // Trim whitespace and skip empty lines
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_string());
            }
        }

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_log_tailer_ignores_existing_entries() {
        // Create a temporary test file
        let test_file = "test_log_tailer.log";

        // Write initial content
        let mut file = fs::File::create(test_file).unwrap();
        writeln!(file, "existing line 1").unwrap();
        writeln!(file, "existing line 2").unwrap();
        drop(file);

        // Create tailer (should seek to end)
        let mut tailer = LogTailer::new(test_file).unwrap();

        // Read should return empty (existing lines ignored)
        let lines = tailer.read_new_lines().unwrap();
        assert_eq!(lines.len(), 0);

        // Append new content
        let mut file = fs::OpenOptions::new().append(true).open(test_file).unwrap();
        writeln!(file, "new line 1").unwrap();
        writeln!(file, "new line 2").unwrap();
        drop(file);

        // Read should return only new lines
        let lines = tailer.read_new_lines().unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "new line 1");
        assert_eq!(lines[1], "new line 2");

        // Clean up
        fs::remove_file(test_file).ok();
    }

    #[test]
    fn test_log_tailer_multiple_reads() {
        let test_file = "test_log_tailer_multiple.log";

        // Create empty file
        fs::File::create(test_file).unwrap();

        // Create tailer
        let mut tailer = LogTailer::new(test_file).unwrap();

        // Append first batch
        let mut file = fs::OpenOptions::new().append(true).open(test_file).unwrap();
        writeln!(file, "batch 1 line 1").unwrap();
        drop(file);

        let lines = tailer.read_new_lines().unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "batch 1 line 1");

        // Append second batch
        let mut file = fs::OpenOptions::new().append(true).open(test_file).unwrap();
        writeln!(file, "batch 2 line 1").unwrap();
        writeln!(file, "batch 2 line 2").unwrap();
        drop(file);

        let lines = tailer.read_new_lines().unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "batch 2 line 1");
        assert_eq!(lines[1], "batch 2 line 2");

        // Read again without new data
        let lines = tailer.read_new_lines().unwrap();
        assert_eq!(lines.len(), 0);

        // Clean up
        fs::remove_file(test_file).ok();
    }
}
