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
	f.set_title(&title.trim());
    let mut content = String::new();
	println!("content: ");
    io::stdin().read_line(&mut content)?;
	f.set_content(&content.trim());
	
	f.generate_hash();
	
	println!("tags (separate by \", \"): ");
    let mut tags = String::new();
    io::stdin().read_line(&mut tags)?;
	for tag in tags.split(", "){
    	let _ = f.add_tag(&tag.trim());
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
        return Ok(());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_argument_success() {
        let args = vec!["todo".to_string(), "add".to_string(), "test".to_string()];
        let result = get_argument(&args, 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "add");
    }

    #[test]
    fn test_get_argument_failure() {
        let args = vec!["todo".to_string()];
        let result = get_argument(&args, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_test_command() {
        let result = test();
        assert!(result.is_ok());
    }

    #[test]
    fn test_view_nonexistent_file() {
        let result = view("nonexistent_hash");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_empty_directory() {
        // Clean up any existing test directory
        let _ = fs::remove_dir_all("test_added");
        
        // Create a test environment
        let original_dir = std::env::current_dir().unwrap();
        let test_dir = original_dir.join("test_list");
        let _ = fs::create_dir(&test_dir);
        std::env::set_current_dir(&test_dir).unwrap();
        
        // Create the added directory but leave it empty
        let _ = fs::create_dir("added");
        
        // Test list command with empty directory
        let args = vec!["todo".to_string(), "list".to_string()];
        let result = list(&args);
        
        // Cleanup
        std::env::set_current_dir(&original_dir).unwrap();
        let _ = fs::remove_dir_all(&test_dir);
        
        // The function should handle the case where directory is empty
        assert!(result.is_ok());
    }

    #[test]
    fn test_help_command() {
        let result = help();
        assert!(result.is_ok());
    }

    #[test]
    fn test_not_found_command() {
        let result = not_found();
        assert!(result.is_ok());
    }

    // Test file format integration
    #[test]
    fn test_todo_file_operations() {
        let mut todo = file_format::TodoFile::new();
        
        // Test setting properties
        todo.set_title("Test Title");
        todo.set_content("Test Content");
        todo.set_time();
        todo.add_tag("test").unwrap();
        todo.add_connection("abc123").unwrap();
        todo.generate_hash();
        
        // Test filename generation
        let filename = todo.filename();
        assert!(filename.ends_with(".todo"));
        
        // Test file string generation
        let file_string = todo.to_file_string();
        assert!(file_string.contains("[title]"));
        assert!(file_string.contains("Test Title"));
        assert!(file_string.contains("[content]"));
        assert!(file_string.contains("Test Content"));
        assert!(file_string.contains("[tags]"));
        assert!(file_string.contains("test"));
        assert!(file_string.contains("[connections]"));
        assert!(file_string.contains("abc123"));
    }
}
