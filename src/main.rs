mod display;
mod file_reader;
mod log_entry;
mod statistics;

use anyhow::{Context, Result};
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use display::DisplayFormatter;
use file_reader::LogTailer;
use log_entry::TraefikLogEntry;
use statistics::StatsCollector;

const LOG_FILE_PATH: &str = "access.log";
const POLL_INTERVAL_SECS: u64 = 3;
const DEFAULT_TOP_N: usize = 10;

fn main() -> Result<()> {
    // Display initial message
    println!("Starting Traefik Access Log Monitor...");
    println!("Looking for: {}", LOG_FILE_PATH);
    println!();

    // Initialize file tailer
    let mut tailer = LogTailer::new(LOG_FILE_PATH)
        .context(format!("Failed to open log file: {}", LOG_FILE_PATH))?;

    println!("✓ Successfully opened log file");
    println!("✓ Ignoring existing entries, monitoring for new requests only");
    println!();
    println!(
        "Starting monitoring loop (polling every {} seconds)...",
        POLL_INTERVAL_SECS
    );
    println!();

    // Wait a moment for user to see the startup messages
    thread::sleep(Duration::from_secs(2));

    // Initialize statistics collector
    let mut stats = StatsCollector::new();

    // Set up channel for receiving top_n updates from input thread
    let (tx, rx) = mpsc::channel();
    let mut top_n = DEFAULT_TOP_N;

    // Spawn input handling thread
    thread::spawn(move || {
        let stdin = io::stdin();
        let reader = stdin.lock();

        for input in reader.lines().map_while(Result::ok) {
            let trimmed = input.trim();
            if let Ok(new_top_n) = trimmed.parse::<usize>()
                && new_top_n > 0
                && new_top_n <= 100
            {
                // Send the new top_n value to the main thread
                if tx.send(new_top_n).is_err() {
                    break; // Main thread has terminated
                }
            }
        }
    });

    // Display initial empty state
    DisplayFormatter::display_stats(&stats, top_n);

    // Main monitoring loop
    loop {
        // Check for top_n updates from input thread (non-blocking)
        while let Ok(new_top_n) = rx.try_recv() {
            top_n = new_top_n;
            // Immediately update display with new top_n
            DisplayFormatter::display_stats(&stats, top_n);
        }

        // Sleep first (poll interval)
        thread::sleep(Duration::from_secs(POLL_INTERVAL_SECS));

        // Read new lines from the log file
        match tailer.read_new_lines() {
            Ok(lines) => {
                if lines.is_empty() {
                    // No new data, but still refresh display
                    DisplayFormatter::display_stats(&stats, top_n);
                    continue;
                }

                // Process each new line
                let mut _parsed_count = 0;
                let mut _error_count = 0;

                for line in lines {
                    match TraefikLogEntry::from_json_line(&line) {
                        Ok(entry) => {
                            stats.add_entry(&entry);
                            _parsed_count += 1;
                        }
                        Err(e) => {
                            // Skip malformed JSON entries
                            _error_count += 1;
                            eprintln!("Warning: Failed to parse log entry: {}", e);
                            eprintln!("Line: {}", line);
                        }
                    }
                }

                // Update display with new statistics
                DisplayFormatter::display_stats(&stats, top_n);

                // Optionally show parse stats in debug mode
                #[cfg(debug_assertions)]
                if _error_count > 0 {
                    eprintln!("Parsed {} entries, {} errors", _parsed_count, _error_count);
                }
            }
            Err(e) => {
                // Handle file read errors
                eprintln!("Error reading log file: {}", e);
                eprintln!("Retrying in {} seconds...", POLL_INTERVAL_SECS);
                DisplayFormatter::display_stats(&stats, top_n);
            }
        }
    }
}
