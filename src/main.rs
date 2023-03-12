mod args;
use args::RustPassArgs;
use clap::Parser;

use rpassword;

use sha2::{Sha512, Digest};

fn main() {
    let args = RustPassArgs::parse();
    let mpass = rpassword::prompt_password(&String::from("Master Password: ")).unwrap();

    let mut hasher = Sha512::new();
    hasher.update(&mpass);
    let result = hasher.finalize();

    println!("Username: {}", args.username);
    println!("service: {}", args.service);
    println!("mpass: {}", mpass);
    println!("hash {:?}", result);
    println!("len: {}", args.len);
    println!("seed: {}", args.seed);
}
