use std::{env,io::{stdin}};

/*
    MODES [CREATE READ]
    create
    read
*/
fn main() {
    let args:Vec<String> = env::args().collect();
    let mode = &args[1];

    print!("Please enter your username : ");
    let mut username = String::new();
    let username = stdin().read_line(&mut username).unwrap();

    print!("Please enter your service : ");
    let mut service = String::new();
    let service = stdin().read_line(&mut service).unwrap();

    print!("Please enter your master password : ");
    let mut mpass = String::new();
    let mpass = stdin().read_line(&mut mpass).unwrap();

    if mode == "create" {
        println!("Create {} {} {}",username,service,mpass);
    } 
    else if mode == "read" {
        println!("Read {} {} {}",username,service,mpass);
    }
    else {
        println!("Unknown Mode");
    }
}
