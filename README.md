# Traefik Access Log Parser

A Windows-compatible Rust application that monitors Traefik access logs in real-time and displays the top N client IPs by request count.

## Features

- **Real-time Monitoring**: Tails the access.log file and processes new entries as they appear
- **Windows Shared Access**: Opens log files with shared read/write mode to avoid blocking Traefik
- **Ignore Existing Entries**: Only tracks requests that occur after the parser starts
- **Runtime Configurable**: Change the number of displayed IPs (1-100) while the parser is running
- **Rich Statistics**: Shows request count, percentage of total, and top 3 accessed paths per IP
- **Clean Display**: Console clears and redraws every 3 seconds with updated statistics
- **Default Display**: Shows top 10 IPs (configurable at runtime)

## Download

Pre-built Windows binaries are available from the [Releases](../../releases) page.

## Building

```bash
cargo build --release
```

The executable will be at `target\release\traefik_log_parser.exe`

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:

### Release Workflow
- **Trigger**: Automatically runs on every push to `main` branch
- **Actions**:
  - Builds Windows x64 release binary
  - Runs all tests
  - Creates GitHub release with auto-generated version tag
  - Uploads build artifacts to the release
- **Artifacts**: Windows executable, README, and sample log

### PR Check Workflow
- **Trigger**: Automatically runs on all pull requests to `main`
- **Actions**:
  - Code formatting check (`cargo fmt`)
  - Linting with clippy (`cargo clippy`)
  - Build (debug and release)
  - Run all tests
  - Documentation check
  - Security audit
- **Purpose**: Ensures code quality before merging

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details on the development workflow.

## Usage

1. Ensure Traefik is configured to write access logs in JSON format to a file named `access.log` in the current directory

2. Run the parser:
   ```bash
   .\target\release\traefik_log_parser.exe
   ```

3. The parser will:
   - Open the access.log file
   - Ignore all existing entries
   - Wait for new log entries to be appended
   - Display top 10 IPs with statistics every 3 seconds (default)

4. **Changing the number of displayed IPs**:
   - While the parser is running, type a number (1-100) and press Enter
   - The display will immediately update to show that many top IPs
   - Example: Type `20` and press Enter to show top 20 IPs

5. Press `Ctrl+C` to exit

## Traefik Configuration

Configure Traefik to write JSON access logs. Example configuration:

```yaml
# traefik.yml
accessLog:
  filePath: "access.log"
  format: json
```

Or via command line:
```bash
--accesslog.filepath=access.log
--accesslog.format=json
```

## Sample Output

```
╔════════════════════════════════════════════════════════════════╗
║        Traefik Access Log Monitor - Top 10 IPs                ║
╚════════════════════════════════════════════════════════════════╝

Total Requests: 1,523 | Unique IPs: 45
Showing top 10 IPs | Type a number and press Enter to change

Top IPs by Request Count:
────────────────────────────────────────────────────────────────

1. 192.168.1.100
   Requests: 456 (29.9%)
   Top Paths:
   • /api/users (234)
   • /api/products (122)
   • /health (100)

2. 10.0.0.50
   Requests: 328 (21.5%)
   Top Paths:
   • /api/orders (200)
   • /api/checkout (128)

3. 172.16.0.25
   Requests: 245 (16.1%)
   Top Paths:
   • /api/search (145)
   • /api/products (100)

...

Press Ctrl+C to exit.
```

### Changing Display Count

While running, you can change how many IPs are displayed:
- Type `5` and press Enter → Shows top 5 IPs
- Type `20` and press Enter → Shows top 20 IPs
- Type `50` and press Enter → Shows top 50 IPs
- Valid range: 1-100

## How It Works

1. **File Tailing**: Opens access.log with Windows shared read/write access
2. **Initial Seek**: Seeks to end of file on startup to ignore existing entries
3. **Polling**: Every 3 seconds, checks for new lines appended to the file
4. **Parsing**: Parses JSON entries to extract ClientHost and RequestPath
5. **Statistics**: Maintains in-memory hash maps tracking request counts and paths per IP
6. **Display**: Clears console and shows top 30 IPs sorted by request count

## Use Case

This tool is designed for real-time traffic analysis during incidents:

1. You notice a spike in Traefik traffic
2. Start this parser locally
3. See which IPs are generating the most requests in real-time
4. Identify patterns in the paths being accessed
5. Take action based on the data (rate limiting, blocking, etc.)

## Testing

Run the included tests:
```bash
cargo test
```

For manual testing with the sample log:
```bash
# Rename sample log to access.log
copy sample_access.log access.log

# Run the parser
.\target\release\traefik_log_parser.exe

# In another terminal, append new entries to simulate Traefik writing logs
echo {"ClientAddr":"192.168.1.200:60000","ClientHost":"192.168.1.200","RequestPath":"/api/test","RequestMethod":"GET","RequestProtocol":"HTTP/1.1","OriginStatus":200,"DownstreamStatus":200} >> access.log
```

## Technical Details

- **Language**: Rust (edition 2024)
- **Dependencies**:
  - `serde` & `serde_json` for JSON parsing
  - `anyhow` for error handling
  - `clearscreen` for console clearing
  - `winapi` for Windows file sharing flags
- **Platform**: Windows (uses Windows-specific file sharing APIs)
- **Performance**: Efficient hash-based statistics tracking with minimal memory overhead
