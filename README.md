# Jukebox-rust

Rust implementation of my jukebox application which allows RFID tags to trigger commands such as opening pictures or play albums.

## Installation

Via crates.io `cargo install jukebox`.

### Dependencies

* SQLite3 (Rusqlite)
* libudev (Serialport)

## WARNING

**I can not stress enough the security implications of not knowing what command will run when a particular tag is swipped. i.e. anyone who can edit or control the database you use can lead you to running commands you would never want to run, or running commands you did not mean to run!**