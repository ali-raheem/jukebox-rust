# Jukebox-rust

Rust application that links RFID tags to shell commands - tap a card to play an album, open a picture, or trigger any action.

GPLv2 | v0.2.0

## Usage

```
jukebox [options]

Options:
    -h, --help          Print this usage information.
    -n, --new           Start new database.
    -a, --add           Add mode, add new action triggers to database.
    -f, --database PATH Database file (default: ./jukebox.db)
    -p, --port PATH     Serial port (default: /dev/ttyACM0)
    -s, --split START:LENGTH
                        Key trimming parameters (default: 3:10)
```

## Building

Requires sqlite3 development libraries:
```bash
# Fedora/RHEL
sudo dnf install libsqlite3x-devel

# Debian/Ubuntu
sudo apt install libsqlite3-dev
```

Build:
```bash
cargo build --release
```

## Running

### Setup: Create database and add cards

```bash
jukebox -n -a
```

This creates a new database and enters add mode. Tap a card on the reader, then enter the command to associate with it. Repeat for each card. Press Ctrl+C when done.

### Production mode

```bash
jukebox -f /etc/jukebox.db
```

### Security

- **Do not run as root** - spawned commands inherit privileges
- Make the database root-owned but world-readable to prevent tampering
- Add your user to the `dialout` group for serial device access:
  ```bash
  sudo usermod -a -G dialout $USER
  ```

## Changelog

See [CHANGELOG.md](CHANGELOG.md)
