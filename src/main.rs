use getopts::Options;
use rusqlite::Connection;
use std::io::{self, BufRead, BufReader};
use std::process::Command;
use std::time::Duration;
use std::{env, fmt};

struct Action {
    cmd: String,
    key: String,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.cmd)
    }
}

impl Action {
    fn exec(self) {
        println!("Action: {}", self.cmd);
        match Command::new("sh")
                             .arg("-c")
                             .arg(self.cmd)
            .status() {
                Ok(n) => println!("Finished, returned {}.", n),
                Err(e) => println!("Failed to run, exit code {}.", e),
            };
    }
}

fn print_usage(name: &str, opts: Options) {
    println!("Jukebox is a program which connects triggers");
    println!("e.g. RFID keys to actions e.g. playing an album.");
    let brief = format!("Usage:\t{} [options]", name);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut prog_opts = Options::new();
    prog_opts.optflag("h", "help", "Print this usage information.");
    prog_opts.optflag("n", "new", "Start new database.");
    prog_opts.optflag("a", "add", "Add mode, add new action triggers to database.");
    prog_opts.optopt("f",
                     "database",
                     "Suggest a name for the database file default ./jukebox.db",
                     "PATH");
    prog_opts.optopt("p",
                     "port",
                     "Serial port to use default /dev/ttyACM0",
                     "PATH");
    prog_opts.optopt("s",
                     "split",
                     "Process key trim first Start chars and continue for length chars default \
                      3:10.",
                     "Start:Length");
    let prog_opts_matches = match prog_opts.parse(&args[1..]) {
        Ok(m) => {
            m
        }
        Err(f) => {
            println!("Fatal Error: Unknown command line option {}", f);
            return;
        }
    };
    if prog_opts_matches.opt_present("h") {
        print_usage(&args[0], prog_opts);
        return;
    }
    let split_str = prog_opts_matches.opt_str("s").unwrap_or_else(|| "3:10".to_owned());
    let split_str_vec: Vec<&str> = split_str.split(":").collect();
    let key_start_char: usize = split_str_vec[0].parse().unwrap();
    let key_length: usize = split_str_vec[1].parse().unwrap();
    let device = prog_opts_matches.opt_str("p").unwrap_or_else(|| "/dev/ttyACM0".to_owned());
    let port = match serialport::new(&device, 9600)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(s) => s,
        Err(_) => {
            println!("Fatal Error: Could not open device.");
            return;
        }
    };
    let mut reader = BufReader::new(port);
    let db_file = prog_opts_matches.opt_str("f").unwrap_or_else(|| "./jukebox.db".to_owned());
    let conn = Connection::open(db_file).unwrap();
    if prog_opts_matches.opt_present("n") {
        conn.execute(
            "CREATE TABLE jukebox (
                cmd TEXT NOT NULL,
                key TEXT KEY
            )",
            [],
        )
        .unwrap();
    }
    if prog_opts_matches.opt_present("a") {
        loop {
            println!("Tap card on reader...\nCtrl+C to exit.");
            let mut input = String::new();
            if reader.read_line(&mut input).is_err() || input.is_empty() {
                continue;
            }
            if input.len() < key_start_char + key_length {
                println!("Card input too short, try again.");
                continue;
            }
            input.drain(..key_start_char);
            input.truncate(key_length);
            println!("Card read: {}. Enter command:", input);
            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).expect("Could not read line from STDIN.");
            let cmd = cmd.trim().to_owned();
            if cmd.is_empty() {
                println!("Empty command, skipping.");
                continue;
            }
            match conn.execute(
                "INSERT INTO jukebox (cmd, key) VALUES ($1, $2)",
                [&cmd, &input],
            ) {
                Ok(_) => {
                    println!("Action added command: {}, trigger: {}.", cmd, input);
                }
                Err(_) => {
                    println!("Failed to add command: {}, trigger:{}.", cmd, input);
                }
            }
        }
    }
    loop {
        let mut input = String::new();
        if reader.read_line(&mut input).is_err() || input.is_empty() {
            continue;
        }
        if input.len() < key_start_char + key_length {
            continue;
        }
        input.drain(..key_start_char);
        input.truncate(key_length);
        println!("Serial device said {}.", input);
        let mut sql_req = match conn.prepare("SELECT cmd, key FROM jukebox WHERE key = (?)") {
            Ok(x) => x,
            Err(_) => {
                continue;
            }
        };
        let action_iter = match sql_req.query_map([&input], |row| {
            Ok(Action {
                cmd: row.get(0)?,
                key: row.get(1)?,
            })
        }) {
            Ok(x) => x,
            Err(_) => {
                continue;
            }
        };
        for action in action_iter {
            match action {
                Ok(trigger) => {
                    println!("Found match: {}.", trigger);
                    trigger.exec();
                }
                Err(_) => {
                    println!("Not an action.")
                }
            }
        }
    }
}
