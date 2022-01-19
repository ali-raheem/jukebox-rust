use std::fmt;
use std::process::Command;

pub struct Action {
    pub cmd: String,
    pub key: String,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.cmd)
    }
}

impl Action {
    pub fn exec(self) {
        println!("Action: {}", self.cmd);
        match Command::new("sh").arg("-c").arg(self.cmd).status() {
            Ok(n) => println!("Finished, returned {}.", n),
            Err(e) => println!("Failed to run, exit code {}.", e),
        };
    }
}
