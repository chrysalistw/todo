use std::env;

mod commands;

fn main() -> std::io::Result<()>{ 
    println!(" -- ToDo List Written in Rust -- ");

    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    println!(" ");

    let command: &str = &args[1];
    if command == "add" {
        commands::add(&args)?;
    }
    else if command == "list" {
        commands::list(&args)?;
    }
    else if command == "view" {
        commands::view(&args)?;
    }
    else if command == "help" {
        commands::help()?;
    }
    else {
        commands::not_found(&args)?;
        //println!("command {} not recognizable.", command);
    }
    Ok(())
}
