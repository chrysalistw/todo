use std::env;

mod commands;

use commands::{get_argument};

fn main() -> std::io::Result<()>{ 
	println!("  _____ ___  ____   ___                   _   ");
	println!(" |_   _/ _ \\|  _ \\ / _ \\   _ __ _   _ ___| |_ ");
	println!("   | || | | | | | | | | | | '__| | | / __| __|");
	println!("   | || |_| | |_| | |_| | | |  | |_| \\__ \\ |_ ");
	println!("   |_| \\___/|____/ \\___/  |_|   \\__,_|___/\\__|");
    println!(" ");
    println!(" -- ToDo List Written in Rust -- ");
    println!(" ");

    let args: Vec<String> = env::args().collect();

    let command = match get_argument(&args, 1){
		Err(_) => "",
		Ok(c) => c
	};
    if command == "add" {
        commands::add()?;
    }
    else if command == "list" {
        commands::list(&args)?;
    }
    else if command == "view" {
        let path = get_argument(&args, 2)?;
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
