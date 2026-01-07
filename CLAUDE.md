# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build release binary
cargo build --release

# Build debug binary
cargo build

# Run tests (note: no unit tests exist currently)
cargo test
```

**Build dependency:** Requires sqlite3 development libraries (e.g., `sudo dnf install lib-sqlite3x-devel` on Fedora).

## Architecture

This is a single-file Rust CLI application (~185 lines in `src/main.rs`) that links serial device inputs (RFID tags) to shell command execution via SQLite storage.

### Core Components

**`Action` struct** (lines 12-34): Holds a command string and RFID key. The `exec()` method spawns the command via `sh -c`.

**Main loop modes:**
- **Add mode** (`-a` flag): Interactive loop that reads RFID tags from serial device and associates them with user-entered shell commands, storing mappings in SQLite.
- **Production mode** (default): Continuous loop reading from serial device, looking up matching actions in the database, and executing associated commands.

### Data Flow

1. Serial device (default `/dev/ttyACM0`) provides raw RFID tag strings
2. Input is trimmed using configurable start position and length (`-s` flag, default `3:10`)
3. SQLite database stores `(cmd, key)` mappings in `jukebox` table
4. Matched commands execute via shell subprocess

### Dependencies

- `rusqlite` - SQLite bindings
- `getopts` - CLI argument parsing
- `serial` - Serial port communication

## CLI Options

```
-h, --help              Print usage
-n, --new               Create new database (creates jukebox table)
-a, --add               Add mode for registering RFID->command mappings
-f, --database PATH     Database file (default: ./jukebox.db)
-p, --port PATH         Serial port (default: /dev/ttyACM0)
-s, --split START:LEN   Key trimming parameters (default: 3:10)
```

## Security Note

Do not run as root - spawned commands inherit privileges. Use udev rules for serial device access instead.
