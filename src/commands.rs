use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::Write;

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
	let mut f = TodoFile::new();

    f.set_time();

    let mut title = String::new();
	println!("title: ");
    io::stdin().read_line(&mut title)?;
	f.set_title(&title.replace("\n", ""));

    let mut content = String::new();
	println!("content: ");
    io::stdin().read_line(&mut content)?;
	f.set_content(&content);
	
	f.generate_hash();
	
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
	let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(file_name)?;
	write!(file, "{}", f.to_file_string())?;

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
        let filename = entry.file_name();
        let hash = filename.to_str().unwrap().replace(".todo", "");
        
        // Read the file to get the title
        let file_path = format!("added/{}.todo", hash);
        let content = fs::read_to_string(&file_path)?;
        let title = content.lines()
            .skip_while(|line| *line != "[title]")
            .nth(1)
            .unwrap_or("No title");
        
        println!("{} ({})", title, hash);
		if listing_contents {
			let _ = view(&hash);
		}
    }
    Ok(())
}
pub fn view(hash: &str) -> std::io::Result<()>{
    let file_path = format!("added/{}.todo", hash);
    let content = fs::read_to_string(&file_path)?;

    // Extract and display title prominently
    let title = content.lines()
        .skip_while(|line| *line != "[title]")
        .nth(1)
        .unwrap_or("No title");
    
    println!("=== {} ===", title);
    println!("{}", content);
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
