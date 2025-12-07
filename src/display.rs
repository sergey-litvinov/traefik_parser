use crate::statistics::StatsCollector;

/// Display formatter for console output
pub struct DisplayFormatter;

impl DisplayFormatter {
    /// Clear the console screen
    pub fn clear_console() {
        if let Err(e) = clearscreen::clear() {
            eprintln!("Failed to clear console: {}", e);
        }
    }

    /// Render the statistics to a formatted string
    pub fn render_stats(stats: &StatsCollector, top_n: usize) -> String {
        let mut output = String::new();

        // Header
        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str(&format!(
            "║        Traefik Access Log Monitor - Top {:2} IPs                ║\n",
            top_n
        ));
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n");
        output.push('\n');

        // Summary stats
        output.push_str(&format!(
            "Total Requests: {} | Unique IPs: {}\n",
            Self::format_number(stats.total_requests()),
            Self::format_number(stats.unique_ips())
        ));
        output.push_str(&format!(
            "Showing top {} IPs | Type a number and press Enter to change\n\n",
            top_n
        ));

        // Check if we have any data
        if stats.total_requests() == 0 {
            output.push_str("⏳ Waiting for log entries...\n");
            output.push_str("\nMonitoring access.log for new requests.\n");
            output.push_str("Press Ctrl+C to exit.\n");
            return output;
        }

        // Get top N IPs
        let top_ips = stats.get_top_ips(top_n);

        output.push_str("Top IPs by Request Count:\n");
        output.push_str("────────────────────────────────────────────────────────────────\n\n");

        // Display each IP
        for (rank, (ip, ip_stats, percentage)) in top_ips.iter().enumerate() {
            // Rank and IP
            output.push_str(&format!("{}. {}\n", rank + 1, ip));

            // Request count and percentage
            output.push_str(&format!(
                "   Requests: {} ({:.1}%)\n",
                Self::format_number(ip_stats.request_count),
                percentage
            ));

            // Top paths
            let top_paths = ip_stats.top_paths(3);
            if !top_paths.is_empty() {
                output.push_str("   Top Paths:\n");
                for (path, count) in top_paths {
                    let truncated_path = Self::truncate_path(&path, 55);
                    output.push_str(&format!(
                        "   • {} ({})\n",
                        truncated_path,
                        Self::format_number(count)
                    ));
                }
            }

            output.push('\n');
        }

        output.push_str("────────────────────────────────────────────────────────────────\n");
        output.push_str("Press Ctrl+C to exit.\n");

        output
    }

    /// Format a number with thousands separators
    fn format_number(n: usize) -> String {
        let s = n.to_string();
        let mut result = String::new();

        for (count, c) in s.chars().rev().enumerate() {
            if count > 0 && count % 3 == 0 {
                result.insert(0, ',');
            }
            result.insert(0, c);
        }

        result
    }

    /// Truncate a path to a maximum length, adding "..." if truncated
    fn truncate_path(path: &str, max_len: usize) -> String {
        if path.len() <= max_len {
            path.to_string()
        } else {
            format!("{}...", &path[..max_len - 3])
        }
    }

    /// Display the statistics (clear console and print)
    pub fn display_stats(stats: &StatsCollector, top_n: usize) {
        Self::clear_console();
        let output = Self::render_stats(stats, top_n);
        println!("{}", output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(DisplayFormatter::format_number(0), "0");
        assert_eq!(DisplayFormatter::format_number(123), "123");
        assert_eq!(DisplayFormatter::format_number(1234), "1,234");
        assert_eq!(DisplayFormatter::format_number(1234567), "1,234,567");
    }

    #[test]
    fn test_truncate_path() {
        assert_eq!(
            DisplayFormatter::truncate_path("/api/users", 20),
            "/api/users"
        );
        assert_eq!(
            DisplayFormatter::truncate_path("/very/long/path/that/exceeds/maximum", 20),
            "/very/long/path/t..."
        );
    }

    #[test]
    fn test_render_empty_stats() {
        let stats = StatsCollector::new();
        let output = DisplayFormatter::render_stats(&stats, 10);
        assert!(output.contains("Waiting for log entries"));
        assert!(output.contains("Total Requests: 0"));
    }
}
