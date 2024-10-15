use std::fs;
use std::io;
use std::io::Result;

pub fn get_argument(args: &[String], index: usize) -> &str{
    match args.get(index){
        None => todo!(),
        Some(path) => path
    }
}
pub fn add(args: &[String]) -> Result<()> {
    let path = match args.get(2){
        None => "",
        Some(path) => path
    };

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
    let command = match args.get(2){
        None => "",
        Some(command) => command
    };
	if command=="-c" {
		listing_contents = true;
		println!("listing with contents");
	}
	else{
    	println!("list");
	}
    let directory_existence: bool = fs::exists("added").unwrap();
    if !directory_existence {
        println!("text directory not found.");
    }
    for entry in fs::read_dir("added")? {
        let entry = entry?;
        println!("{:?}", entry.file_name());
		if listing_contents {
			view(std::slice::from_ref(&entry.file_name().into_string().unwrap()));
		}
    }
    Ok(())
}
pub fn view(args: &[String]) -> std::io::Result<()>{
    let path = match args.get(2){
        None => todo!(),
        Some(path) => path
    };

    let file_path = format!("added/{}.txt", path);
    let content = fs::read(&file_path)?;

    //println!("viewing {}.txt: ", &path);
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
