mod args;
use args::RustPassArgs;
use clap::Parser;

fn main() {
    let args = RustPassArgs::parse(); 

    println!("Username : {}",args.username);
    println!("service : {}",args.service);
    println!("mpass : {}",args.mpass);
    println!("len : {}",args.len);
    println!("seed : {}",args.seed);
}
