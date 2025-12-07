use crate::log_entry::TraefikLogEntry;
use std::collections::HashMap;

/// Statistics for a single IP address
#[derive(Debug, Clone)]
pub struct IpStats {
    /// Total number of requests from this IP
    pub request_count: usize,
    /// Map of request paths to their access counts
    pub paths: HashMap<String, usize>,
}

impl IpStats {
    /// Create new empty statistics for an IP
    pub fn new() -> Self {
        IpStats {
            request_count: 0,
            paths: HashMap::new(),
        }
    }

    /// Add a request to this IP's statistics
    pub fn add_request(&mut self, path: &str) {
        self.request_count += 1;
        *self.paths.entry(path.to_string()).or_insert(0) += 1;
    }

    /// Get the top N most accessed paths by this IP
    /// Returns vector of (path, count) tuples sorted by count descending
    pub fn top_paths(&self, n: usize) -> Vec<(String, usize)> {
        let mut paths: Vec<(String, usize)> = self
            .paths
            .iter()
            .map(|(path, count)| (path.clone(), *count))
            .collect();

        // Sort by count descending
        paths.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top N
        paths.into_iter().take(n).collect()
    }
}

/// Collector for all IP statistics
pub struct StatsCollector {
    /// Map of IP addresses to their statistics
    stats: HashMap<String, IpStats>,
    /// Total number of requests tracked
    total_requests: usize,
}

impl StatsCollector {
    /// Create a new empty statistics collector
    pub fn new() -> Self {
        StatsCollector {
            stats: HashMap::new(),
            total_requests: 0,
        }
    }

    /// Add a log entry to the statistics
    pub fn add_entry(&mut self, entry: &TraefikLogEntry) {
        // Extract IP and path
        let ip = match entry.get_ip() {
            Some(ip) => ip,
            None => return, // Skip entries without IP
        };

        let path = entry.get_path();

        // Update or create IP stats
        let ip_stats = self.stats.entry(ip).or_insert_with(IpStats::new);
        ip_stats.add_request(&path);

        // Increment total requests
        self.total_requests += 1;
    }

    /// Get the top N IPs by request count
    /// Returns vector of (ip, stats, percentage) tuples sorted by request count descending
    pub fn get_top_ips(&self, n: usize) -> Vec<(String, &IpStats, f64)> {
        let mut ips: Vec<(String, &IpStats, f64)> = self
            .stats
            .iter()
            .map(|(ip, stats)| {
                let percentage = if self.total_requests > 0 {
                    (stats.request_count as f64 / self.total_requests as f64) * 100.0
                } else {
                    0.0
                };
                (ip.clone(), stats, percentage)
            })
            .collect();

        // Sort by request count descending
        ips.sort_by(|a, b| b.1.request_count.cmp(&a.1.request_count));

        // Take top N
        ips.into_iter().take(n).collect()
    }

    /// Get the total number of requests tracked
    pub fn total_requests(&self) -> usize {
        self.total_requests
    }

    /// Get the total number of unique IPs tracked
    pub fn unique_ips(&self) -> usize {
        self.stats.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_stats_add_request() {
        let mut stats = IpStats::new();
        stats.add_request("/api/users");
        stats.add_request("/api/users");
        stats.add_request("/api/products");

        assert_eq!(stats.request_count, 3);
        assert_eq!(stats.paths.get("/api/users"), Some(&2));
        assert_eq!(stats.paths.get("/api/products"), Some(&1));
    }

    #[test]
    fn test_ip_stats_top_paths() {
        let mut stats = IpStats::new();
        stats.add_request("/path1");
        stats.add_request("/path2");
        stats.add_request("/path2");
        stats.add_request("/path3");
        stats.add_request("/path3");
        stats.add_request("/path3");

        let top = stats.top_paths(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0], ("/path3".to_string(), 3));
        assert_eq!(top[1], ("/path2".to_string(), 2));
    }

    #[test]
    fn test_stats_collector_percentage() {
        let mut collector = StatsCollector::new();

        // Create mock entries
        let entry1 = TraefikLogEntry {
            client_host: Some("192.168.1.1".to_string()),
            client_addr: None,
            request_path: Some("/api/test".to_string()),
            request_method: None,
            request_protocol: None,
            origin_status: None,
            downstream_status: None,
        };

        let entry2 = TraefikLogEntry {
            client_host: Some("192.168.1.2".to_string()),
            client_addr: None,
            request_path: Some("/api/test".to_string()),
            request_method: None,
            request_protocol: None,
            origin_status: None,
            downstream_status: None,
        };

        // Add entries: 3 from IP1, 1 from IP2
        collector.add_entry(&entry1);
        collector.add_entry(&entry1);
        collector.add_entry(&entry1);
        collector.add_entry(&entry2);

        assert_eq!(collector.total_requests(), 4);
        assert_eq!(collector.unique_ips(), 2);

        let top_ips = collector.get_top_ips(10);
        assert_eq!(top_ips.len(), 2);
        assert_eq!(top_ips[0].0, "192.168.1.1");
        assert_eq!(top_ips[0].1.request_count, 3);
        assert!((top_ips[0].2 - 75.0).abs() < 0.01); // 75%
    }
}
