# Jukebox-rust

Rust implementation of a jukebox application which allows RFID tags to trigger commands such as opening pictures or play albums.

```
ali@ali-K53E:~/Code/Rust/jukebox$ ./target/debug/jukebox -h
Jukebox is a program which connects triggers e.g. RFID keys to actions e.g. playing an album.
Usage:	./target/debug/jukebox [options]

Options:
    -f, --database PATH Suggest a name for the database file default
                        ./jukebox.db
    -h, --help          Print this usage information.
    -n, --new           Start new database.
    -a, --add           Add mode, add new action triggers to database.
    -p, --port PATH     Serial port to use default /dev/ttyACM0
```

Example usage would be putting a RFID tag/card in a CD case and using it to trigger your computer to play said album.

## ToDo
Fix add mode.
