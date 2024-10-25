use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::Write;
use std::path::Path;
use chrono;

mod file_format;
use file_format::{TodoFile};


pub fn get_argument(args: &[String], index: usize) -> std::io::Result<&str>{
    let argument_not_found = Error::new(ErrorKind::InvalidInput, "argument not found.");
    match args.get(index){
        Some(path) => Ok(path),
        None => Err(argument_not_found)
    }
}
pub fn add(path: &str) -> std::io::Result<()> {
	let time_stamp = chrono::offset::Local::now();
	println!("current time is: {:#?}", time_stamp);
    println!("adding to {}", path);
    println!("content of txt: ");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let directory_existence: bool = fs::exists("added").unwrap();
    if !directory_existence {
        fs::create_dir("added")?;
    }
    
    let file_name = format!("added/{}.txt", path);

	let mut file = OpenOptions::new().create(true).write(true).append(true).open(&file_name)?;

	writeln!(file, "{:?}", time_stamp)?;
	writeln!(file, "{}", buffer)?;
    Ok(())
}
pub fn test() -> std::io::Result<()>{
    let mut serial_num_String: String = String::from_utf8(fs::read("info.txt")?).unwrap();
    if serial_num_String.ends_with('\n') {
        serial_num_String.pop();
        if serial_num_String.ends_with('\r') {
            serial_num_String.pop();
        }
    }
	println!("String: {}", serial_num_String);
	let serial_num: u32 = serial_num_String.parse().unwrap();
	println!("num: {}", serial_num);
	let mut f = TodoFile::new();
	println!("{:?}", f);

	f.set_number(serial_num+1);
    f.set_time();
	f.set_content("abc_abc");
    let _ = f.add_tag("test_a");
    let _ = f.add_tag("test_b");
    let _ = f.add_tag("test_c");
    let _ = f.remove_tag("test_b");

	println!("{:?}", f);
	f.to_file_string();
	let mut info = OpenOptions::new().create(true).write(true).open("info.txt")?;
	write!(info, "{}", serial_num+1)?;
	Ok(())
}
pub fn list(args: &[String]) -> std::io::Result<()>{
	let listing_contents: bool;

    let flag = get_argument(&args, 2)?;
    match flag {
        "-c" => { listing_contents = true; },
        &_ => { listing_contents = false; }
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

    println!("{}", String::from_utf8(content).unwrap());
    Ok(())
}
pub fn edit(_args: &[String]) -> std::io::Result<()>{
	todo!();
}
pub fn help() -> std::io::Result<()>{
    println!("Listing available commands:");
    println!(" add [filename]");
    println!("     append content to [filename].txt with timestamp.");
    println!(" list ");
    println!("     list files in added/. Attach `-c` to show their contents.");
    println!(" view [filename]");
    println!("     view content of [filename].txt");
    println!(" help ");
    println!("     you're looking at it right now.");
    Ok(())
}
pub fn not_found() -> std::io::Result<()>{
    println!("command not recognizable.");
    println!("type `todo help` to see instructions.");
    Ok(())
}
