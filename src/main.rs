use std::{env, io::stdin};

/*
    MODES [CREATE READ]
    create
    read
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];

    println!("Please enter your username : ");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();

    println!("Please enter your service : ");
    let mut service = String::new();
    stdin().read_line(&mut service).unwrap();

    println!("Please enter your master password : ");
    let mut mpass = String::new();
    stdin().read_line(&mut mpass).unwrap();

    if mode == "create" {
        println!(
            "Create {} {} {}",
            username.trim(),
            service.trim(),
            mpass.trim()
        );
    } else if mode == "read" {
        println!("Read {} {} {}", username, service, mpass);
    } else {
        println!("Unknown Mode");
    }
}
