# Dawn Checker 🎯

A fast and reliable Roblox cookie validation tool built in Rust. It helps you check if your Roblox accounts are still valid, banned, or have issues - either through batch processing of cookie files or real-time monitoring with a nice terminal interface.

## Features

### Batch Checking (Mode 1)
- **Cookie Authentication**: Checks if your Roblox cookies are valid
- **Ban Detection**: Quickly identifies banned accounts
- **Friends Count**: Optionally fetch friend counts for live accounts
- **Smart Filtering**: Filter accounts based on friend count ranges
- **Clean Output**: Shows results in a nice table with progress bars
- **File Export**: Save valid cookies with timestamps

### Real-Time Monitoring (Mode 2)
- **Live Dashboard**: Beautiful terminal UI that updates in real-time
- **Continuous Monitoring**: Automatically re-checks all cookies at set intervals
- **Account Statistics**: See live counts broken down by status
- **Presence Tracking**: Monitor which accounts are online/offline/in-game
- **System Info**: Display CPU usage, RAM, and ping times
- **Activity Logs**: Scroll through recent check results
- **Easy Controls**: Press 'q' or Escape to quit

## Technical Details

- **Written in**: Rust 2024
- **Async Runtime**: Tokio for handling multiple requests efficiently
- **HTTP Client**: Reqwest with cookie support
- **Terminal UI**: Ratatui with Crossterm for cross-platform compatibility
- **File Picker**: Native file dialogs
- **System Monitoring**: Hardware stats via sysinfo, ping with icmp
- **Concurrency**: Multi-threaded processing with configurable thread counts

## Installation

### Requirements
- Rust 1.80.0+
- Cargo

### Build from Source
```bash
git clone <repository-url>
cd check_online
cargo build --release
```

You'll find the executable in `target/release/check_online` (Linux/Mac) or `target/release/check_online.exe` (Windows).

## How to Use

Run the program and you'll see a cool ASCII title:
```bash
./check_online  # or check_online.exe on Windows
```

It gives you two main options:

### Option 1: Batch Check Cookies
Perfect for quickly validating a bunch of cookies from a file.

**Steps:**
1. **Set Thread Count**: Enter how many threads to use (default: 10, up to what makes sense for your machine)
2. **Pick Cookie File**: Browse and select your .txt file with Roblox cookies
3. **Wait for Processing**:
   - First it checks if each cookie can authenticate
   - Then verifies ban status (with rate limiting to be nice)
   - Optionally gets friend counts for valid accounts
4. **View Results**: See a table with breakdown of LIVE/DEAD/BANNED/ERROR accounts
5. **Optional Filtering**: If you have live accounts, you can filter by friend count ranges
6. **Save Results**: Export just the valid cookies to a new file

**Cookie File Format:**
```
_ROBLOSECURITY=your_cookie_here
_ROBLOSECURITY=another_cookie
```

Empty lines between cookies work too.

### Option 2: Real-Time Monitoring
Great for keeping tabs on important accounts over time.

**Steps:**
1. **Select Cookie File**: Pick your cookie file
2. **Set Check Interval**: How often to re-check (60-360 seconds)
3. **Watch the Dashboard**: The terminal UI shows you live stats

The interface has three panels:
- **Stats**: Current counts of each account type
- **Presence**: Online/offline status for valid accounts
- **Hardware**: Your computer's CPU/RAM usage and network ping
- **Logs**: Recent activity with timestamps
- **Countdown**: Time until next check cycle

Use 'q' or Escape to exit when done.

## Project Structure

```
src/
├── main.rs                 # Main menu and entry point
├── check/                  # Batch processing
│   ├── mod.rs             # Module setup
│   ├── main.rs            # Coordinates batch operations
│   ├── data.rs            # Data structures for batch checks
│   ├── models.rs          # Result models and tables
│   ├── types.rs           # Roblox API response types
│   ├── filter.rs          # Friend count filtering
│   └── phase/             # Checking phases
│       ├── mod.rs
│       ├── phase1.rs      # Authentication
│       ├── phase2.rs      # Ban checking
│       └── phase3.rs      # Friend count fetching
└── check_realtime/        # Live monitoring
    ├── mod.rs             # Module setup
    ├── main.rs            # TUI setup and main loop
    ├── data.rs            # Real-time data structures
    ├── checker.rs         # Cookie validation logic
    ├── ui.rs              # Terminal interface rendering
    └── utils.rs           # Hardware monitoring
```

## Important Notes

⚠️ **Please be responsible:**

1. **Your Cookies Only**: Only use cookies from accounts you actually own
2. **Rate Limiting**: The tool automatically respects Roblox API limits (uses RoProxy when rate limited)
3. **Legitimate Use**: This is for managing your own accounts, not for automation or rule-breaking
4. **No Game Automation**: Doesn't (and shouldn't) automate gameplay or bypass rules

The developers aren't responsible for misuse. Always follow Roblox's terms of service.

## Dependencies

### Core
- `tokio` - Async operations
- `reqwest` - HTTP requests with cookies
- `ratatui` + `crossterm` - Terminal UI
- `serde` - Data serialization
- `sysinfo` + `ping-rs` - System/network monitoring

### Utilities
- `rfd` - File picker dialogs
- `tabled` - Pretty tables
- `chrono` - Time handling
- `indicatif` - Progress bars
- `figlet-rs` - ASCII art
- `ansi_term` - Colors

## Development

Build debug version:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Format code:
```bash
cargo fmt
```

Check for issues:
```bash
cargo clippy
```

## Contributing

We welcome contributions! Here's how:

1. Fork this repository
2. Create a new branch for your feature: `git checkout -b feature-name`
3. Make your changes (please write tests!)
4. Test everything works: `cargo test`
5. Submit a pull request

## License

This project is open source under the MIT License - feel free to use it in your own projects.

See the [LICENSE](LICENSE) file for full details.

## Disclaimer

Use at your own risk. We don't take responsibility for any misuse or violations of platform terms of service. Always make sure your usage complies with applicable rules and laws.
