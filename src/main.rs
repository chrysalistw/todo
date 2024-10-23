use std::env;

mod commands;

use commands::{get_argument};

fn main() -> std::io::Result<()>{ 
    println!(" -- ToDo List Written in Rust -- ");
    println!(" ");

    let args: Vec<String> = env::args().collect();

    let command = get_argument(&args, 1).unwrap();
    if command == "add" {
        let path = get_argument(&args, 2).unwrap();
        commands::add(path)?;
    }
    else if command == "list" {
        commands::list(&args)?;
    }
    else if command == "view" {
        let path = get_argument(&args, 2).unwrap();
        commands::view(path)?;
    }
    else if command == "edit" {
        commands::edit(&args)?;
    }
	else if command == "test" {
		commands::test()?;
	}
    else if command == "help" {
        commands::help()?;
    }
    else {
        commands::not_found()?;
    }
    Ok(())
}
