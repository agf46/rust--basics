use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

fn main() {
    loop {
        // use the > character as a prompt 
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // dont accept another command until the previous command is over 
        child.wait();
    }
}