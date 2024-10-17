use std::env;

mod commands;

use commands::{get_argument};

fn main() -> std::io::Result<()>{ 
    println!(" -- ToDo List Written in Rust -- ");

    let args: Vec<String> = env::args().collect();
    println!(" ");

    let command = get_argument(&args, 1);
    if command == "add" {
        let path = get_argument(&args, 2);
        commands::add(path)?;
    }
    else if command == "list" {
        commands::list(&args)?;
    }
    else if command == "view" {
        let path = get_argument(&args, 2);
        commands::view(path)?;
    }
    else if command == "edit" {
        commands::edit(&args)?;
    }
    else if command == "help" {
        commands::help()?;
    }
    else {
        commands::not_found()?;
    }
    Ok(())
}
