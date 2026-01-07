# Jukebox-rust

Rust application that links RFID tags to scripts - tap a card to play an album, open a picture, or trigger any action.

GPLv2 | v0.3.0

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
    -d, --scripts PATH  Script directory (default: /etc/jukebox.d)
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

## Setup

### 1. Create script directory

```bash
sudo mkdir -p /etc/jukebox.d
sudo chown root:root /etc/jukebox.d
sudo chmod 755 /etc/jukebox.d
```

### 2. Add scripts

Create executable scripts for each action. Example:

```bash
# /etc/jukebox.d/play_jazz
#!/bin/bash
mpv /music/jazz/
```

Make scripts executable:
```bash
sudo chmod +x /etc/jukebox.d/*
```

### 3. Create database and register cards

```bash
jukebox -n -a
```

This creates a new database and enters add mode. The available scripts are listed. Tap a card, then enter the script name to associate with it. Repeat for each card. Press Ctrl+C when done.

### 4. Run in production

```bash
jukebox -f /etc/jukebox.db
```

## Security

This version uses a **script directory approach** instead of arbitrary shell commands:

- Only scripts in the designated directory (`/etc/jukebox.d`) can be executed
- Script names cannot contain path separators (no `../` attacks)
- Scripts are executed directly (no shell interpretation)

Recommendations:
- **Do not run jukebox as root** - spawned scripts inherit privileges
- Make scripts root-owned: `sudo chown root:root /etc/jukebox.d/*`
- Make database root-owned but world-readable
- Add your user to the `dialout` group for serial device access:
  ```bash
  sudo usermod -a -G dialout $USER
  ```

## Migrating from 0.2.x

Version 0.3.0 introduces breaking changes:

1. Commands are no longer stored in the database - only script names
2. You must create scripts in `/etc/jukebox.d/` (or custom `-d` path)
3. Re-register your cards with `jukebox -a` using script names

## Changelog

See [CHANGELOG.md](CHANGELOG.md)
