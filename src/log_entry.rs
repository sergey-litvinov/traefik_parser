use serde::Deserialize;

/// Represents a Traefik access log entry in JSON format
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TraefikLogEntry {
    #[serde(rename = "ClientAddr")]
    pub client_addr: Option<String>,

    #[serde(rename = "ClientHost")]
    pub client_host: Option<String>,

    #[serde(rename = "RequestPath")]
    pub request_path: Option<String>,

    #[serde(rename = "RequestMethod")]
    pub request_method: Option<String>,

    #[serde(rename = "RequestProtocol")]
    pub request_protocol: Option<String>,

    #[serde(rename = "OriginStatus")]
    pub origin_status: Option<u16>,

    #[serde(rename = "DownstreamStatus")]
    pub downstream_status: Option<u16>,
}

impl TraefikLogEntry {
    /// Parse a JSON line into a TraefikLogEntry
    pub fn from_json_line(line: &str) -> anyhow::Result<Self> {
        let entry: TraefikLogEntry = serde_json::from_str(line)?;
        Ok(entry)
    }

    /// Extract the IP address from the log entry
    /// Prefers ClientHost, falls back to ClientAddr (removing port if present)
    pub fn get_ip(&self) -> Option<String> {
        // Try ClientHost first
        if let Some(ref host) = self.client_host
            && !host.is_empty()
        {
            return Some(host.clone());
        }

        // Fall back to ClientAddr, strip port if present
        if let Some(ref addr) = self.client_addr {
            if let Some(colon_pos) = addr.rfind(':') {
                // Extract IP part before the port
                return Some(addr[..colon_pos].to_string());
            }
            return Some(addr.clone());
        }

        None
    }

    /// Get the request path, defaulting to "/" if not present
    pub fn get_path(&self) -> String {
        self.request_path.clone().unwrap_or_else(|| "/".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_entry() {
        let json =
            r#"{"ClientHost":"192.168.1.100","RequestPath":"/api/users","OriginStatus":200}"#;
        let entry = TraefikLogEntry::from_json_line(json).unwrap();
        assert_eq!(entry.get_ip(), Some("192.168.1.100".to_string()));
        assert_eq!(entry.get_path(), "/api/users");
    }

    #[test]
    fn test_extract_ip_from_client_addr() {
        let json = r#"{"ClientAddr":"10.0.0.1:54321","RequestPath":"/test"}"#;
        let entry = TraefikLogEntry::from_json_line(json).unwrap();
        assert_eq!(entry.get_ip(), Some("10.0.0.1".to_string()));
    }

    #[test]
    fn test_missing_path_defaults_to_slash() {
        let json = r#"{"ClientHost":"192.168.1.1"}"#;
        let entry = TraefikLogEntry::from_json_line(json).unwrap();
        assert_eq!(entry.get_path(), "/");
    }
}
