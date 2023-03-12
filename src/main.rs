mod args;
use args::RustPassArgs;
use clap::Parser;
use rpassword;

fn main() {
    let args = RustPassArgs::parse();
    let mpass = rpassword::prompt_password(&String::from("Master Password: ")).unwrap();

    println!("Username : {}", args.username);
    println!("service : {}", args.service);
    println!("mpass : {}", mpass);
    println!("len : {}", args.len);
    println!("seed : {}", args.seed);
}
