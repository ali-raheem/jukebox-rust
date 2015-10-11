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

#### Building
Uses the cargo buildsystem for dependencies and building.
To build run:
```
cargo build --release
```

### Running
It is suggested to make a database like in the current directory and then copy it to /etc and make it root writeable but world readable.
Then to run jukebox *should not* run as root.
Use a udev rule to make the serial device readble by the user you run jukebox as.

###### ToDo
Fix add mode.
