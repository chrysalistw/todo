use std::fs;
use std::io;
use std::io::Result;
use std::path::Path;

pub fn get_argument(args: &[String], index: usize) -> &str{
    match args.get(index){
        None => todo!(),
        Some(path) => path
    }
}
pub fn add(path: &str) -> Result<()> {
    println!("adding to {}", path);
    println!("content of txt: ");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let directory_existence: bool = fs::exists("added").unwrap();
    if !directory_existence {
        fs::create_dir("added")?;
    }
    
    let file_name = format!("added/{}.txt", path);
    fs::write(file_name, buffer)?;
    Ok(())
}
pub fn list(args: &[String]) -> std::io::Result<()>{
	let mut listing_contents = false;

    let flag = get_argument(&args, 2);
    match flag {
        "-c" => { listing_contents = true; },
        &_ => todo!()
    }
    let directory_existence: bool = fs::exists("added").unwrap();
    if !directory_existence {
        println!("text directory not found.");
    }
    for entry in fs::read_dir("added")? {
        let entry = entry?;
        println!("{:?}", entry.file_name());
		if listing_contents {
			let _ = view(entry.file_name().to_str().unwrap());
		}
    }
    Ok(())
}
pub fn view(path: &str) -> std::io::Result<()>{
    let path = Path::new(path).file_stem().unwrap().to_str().unwrap();
    let file_path = format!("added/{}.txt", path);
    let content = fs::read(&file_path)?;

    //println!("viewing {}.txt: ", path);
    println!("{}", String::from_utf8(content).unwrap());
    Ok(())
}
pub fn edit(args: &[String]) -> std::io::Result<()>{
	todo!();
}
pub fn help() -> std::io::Result<()>{
    println!("Listing available commands:");
    println!(" add [filename]");
    println!("     adds content to [filename].txt");
    println!(" list ");
    println!("     list files in added/");
    println!(" view [filename]");
    println!("     view content of [filename].txt");
    println!(" help ");
    println!("     you're looking at it right now.");
    Ok(())
}
pub fn not_found(args: &[String]) -> std::io::Result<()>{
    let command = &args[1];
    println!("command {} not recognizable.", command);
    println!("type `todo help` to see instructions.");
    Ok(())
}
