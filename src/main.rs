use std::{io::stdin, process::Command};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let cmd = input.trim(); // remove a trailing newline
    Command::new(cmd).spawn().unwrap();
    println!("'{}' executed", cmd)
}
