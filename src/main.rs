use std::env;
use std::process;
// use std::io::stdin;

use rustpass::Flags;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags = Flags::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("username: {}", flags.username);
    println!("service: {}", flags.service);
    println!("master-password: {}", flags.md);
    println!("opts: {}", flags.opts);
    println!("length: {}", flags.len);
    println!("seed: {}", flags.seed);
}
