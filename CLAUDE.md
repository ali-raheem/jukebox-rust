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

# Run clippy
cargo clippy
```

**Build dependency:** Requires sqlite3 development libraries (e.g., `sudo dnf install lib-sqlite3x-devel` on Fedora, `sudo apt install libsqlite3-dev` on Debian/Ubuntu).

## Architecture

Single-file Rust CLI application (`src/main.rs`) that links RFID tags to script execution via SQLite storage.

### Security Model

Uses a **script directory approach** - only scripts in a designated directory (`/etc/jukebox.d` by default) can be executed. Script names are validated to prevent path traversal.

### Core Components

**`is_valid_script_name()`**: Validates script names - rejects paths containing `/`, `\`, `..`, or null bytes.

**`Action` struct**: Holds script name and RFID key. The `exec()` method validates the script name, checks the script exists, then executes it directly (no shell).

**Main loop modes:**
- **Add mode** (`-a` flag): Lists available scripts, reads RFID tags, associates them with script names in SQLite.
- **Production mode** (default): Reads RFID tags, looks up scripts in database, executes matching scripts.

### Data Flow

1. Serial device (default `/dev/ttyACM0`) provides raw RFID tag strings
2. Input is trimmed using configurable start position and length (`-s` flag, default `3:10`)
3. SQLite database stores `(script_name, key)` mappings in `jukebox` table
4. Script is executed directly from script directory (no shell interpretation)

### Dependencies

- `rusqlite` - SQLite bindings
- `getopts` - CLI argument parsing
- `serialport` - Serial port communication

## CLI Options

```
-h, --help              Print usage
-n, --new               Create new database
-a, --add               Add mode for registering RFID->script mappings
-f, --database PATH     Database file (default: ./jukebox.db)
-p, --port PATH         Serial port (default: /dev/ttyACM0)
-s, --split START:LEN   Key trimming parameters (default: 3:10)
-d, --scripts PATH      Script directory (default: /etc/jukebox.d)
```

## Security Notes

- Scripts executed directly without shell interpretation
- Path traversal attacks prevented by script name validation
- Do not run as root - spawned scripts inherit privileges
- Scripts should be root-owned in a root-owned directory
