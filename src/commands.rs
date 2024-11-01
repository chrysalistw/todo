use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::Write;
use std::path::Path;

mod file_format;
use file_format::{TodoFile};


pub fn get_argument(args: &[String], index: usize) -> std::io::Result<&str>{
    let argument_not_found = Error::new(ErrorKind::InvalidInput, "argument not found.");
    match args.get(index){
        Some(path) => Ok(path),
        None => Err(argument_not_found)
    }
}
pub fn test() -> std::io::Result<()>{
	Ok(())
}
pub fn add() -> std::io::Result<()>{
	let serial_num: usize = match fs::read_to_string(".serial_num"){
		Ok(s) => s.parse().unwrap(),
		Err(_) => 0
	};

	let mut f = TodoFile::new();

	f.set_number(serial_num+1);
    f.set_time();

    let mut content = String::new();
	println!("content: ");
    io::stdin().read_line(&mut content)?;
	f.set_content(&content);
	println!("tags (separate by \", \"): ");
    let mut tags = String::new();
    io::stdin().read_line(&mut tags)?;
	for tag in tags.split(", "){
    	let _ = f.add_tag(&tag.replace("\n", ""));
	}

    let directory_existence: bool = fs::exists("added").unwrap();
    if !directory_existence {
        fs::create_dir("added")?;
    }

    let file_name = format!("added/{}", f.filename());
	let mut file = OpenOptions::new().create(true).write(true).append(true).open(file_name)?;
	write!(file, "{}", f.to_file_string())?;

	let mut serial_num_file = OpenOptions::new().create(true).write(true).open(".serial_num")?;
	write!(serial_num_file, "{}", serial_num+1)?;
	Ok(())
}
pub fn list(args: &[String]) -> std::io::Result<()>{
	let listing_contents: bool = match get_argument(&args, 2) {
		Err(_) => false,
		Ok(flag) => flag == "-c"
	};

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
    let file_path = format!("added/{}.todo", path);
    let content = fs::read(&file_path)?;

    println!("{}", String::from_utf8(content).unwrap());
    Ok(())
}
pub fn edit(_args: &[String]) -> std::io::Result<()>{
	todo!();
}
pub fn help() -> std::io::Result<()>{
    println!("Listing available commands:");
    println!(" add");
    println!("     add a todo file.");
    println!(" list ");
    println!("     list files in added/. Attach `-c` to show their contents.");
    println!(" view [filename]");
    println!("     view content of [filename].todo");
    println!(" help ");
    println!("     you're looking at it right now.");
    Ok(())
}
pub fn not_found() -> std::io::Result<()>{
    println!("command not recognizable.");
    println!("type `todo help` to see instructions.");
    Ok(())
}
