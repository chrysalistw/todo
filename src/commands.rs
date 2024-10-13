use std::fs;
//use std::path;
use std::io;
use std::io::Result;

fn get_command(args: &[String], index: usize) -> Result<&str, String> {
    if args.len()>index {
        Ok(&args[index])
    }
    else{
        Err(format!("Index {} out of bounds. Max index: {}", index, args.len()-1))
    }
}
pub fn add(args: &Vec<String>) -> Result<()> {
    let path: &str;
    if args.len()>=3 {
        path = &args[2]
    }
    else {
        println!("path unspecified. panicking.");
        panic!();
    }

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
pub fn list(_args: &Vec<String>) -> std::io::Result<()>{
    println!("list");
    let directory_existence: bool = fs::exists("added").unwrap();
    if !directory_existence {
        println!("text directory not found.");
    }
    for entry in fs::read_dir("added")? {
        let entry = entry?;
        println!("{:?}", entry.file_name());
    }
    Ok(())
}
pub fn view(args: &Vec<String>) -> std::io::Result<()>{
    let path: &str;
    if args.len()>=3 {
        path = &args[2];
    }
    else {
        println!("path unspecified. panicking.");
        panic!();
    }

    let file_path = format!("added/{}.txt", path);
    let content = fs::read(&file_path)?;

    println!("viewing {}.txt: ", &path);
    println!("{}", String::from_utf8(content).unwrap());
    Ok(())
}
pub fn not_found(args: &Vec<String>) -> std::io::Result<()>{
    let command = &args[1];
    println!("command {} not recognizable.", command);
    Ok(())
}
