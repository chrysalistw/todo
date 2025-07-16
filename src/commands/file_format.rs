use chrono;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
#[allow(dead_code)]
pub struct TodoFile {
		hash: String,
		title: String,
		time: String,
		content: String,
		tags: Vec<String>,
		connections: Vec<String>,
}

impl TodoFile {
	pub fn new() -> Self {
		Self {
			hash: String::from(""),
			title: String::from(""),
			time: String::from(""),
			content: String::from(""),
			tags: vec!(),
			connections: vec!(),
		}
	}

	pub fn set_title(&mut self, title: &str){
		if self.title != "" {
			println!("title already exists: {}", self.title);
			return;
		}
		self.title = title.to_string();
	}

	pub fn generate_hash(&mut self) {
		let mut hasher = DefaultHasher::new();
		format!("{}{}", self.content, self.time).hash(&mut hasher);
		self.hash = format!("{:x}", hasher.finish());
	}

	pub fn set_time(&mut self){
	    let time_stamp = chrono::offset::Local::now();
        self.time = format!("{}", time_stamp);
	}

	pub fn set_content(&mut self, content: &str){
		if self.content != "" {
			println!("content already exists: {}", self.content);
			return;
		}
		self.content = content.to_string();
	}

	pub fn add_tag(&mut self, tag: &str) -> Result<(), &'static str> {
        let index = self.tags.iter().position(|r| r == tag);
        match index {
            None => { self.tags.push(tag.to_string()); },
            Some(_) => { return Err("tag already exists"); }
        }
        Ok(())
	}

	#[allow(dead_code)]
	pub fn remove_tag(&mut self, tag: &str) -> Result<(), &'static str> {
        let index = self.tags.iter().position(|r| r == tag);
        match index {
            None => { return Err("tag does not exist"); },
            Some(i) => { self.tags.remove(i); }
        }
        Ok(())
	}

	#[allow(dead_code)]
	pub fn list_tags(&mut self) {
        println!("{:#?}", self.tags);
	}

	pub fn add_connection(&mut self, connection_hash: &str) -> Result<(), &'static str> {
        let index = self.connections.iter().position(|r| r == connection_hash);
        match index {
            None => { self.connections.push(connection_hash.to_string()); },
            Some(_) => { return Err("connection already exists"); }
        }
        Ok(())
	}

	#[allow(dead_code)]
	pub fn remove_connection(&mut self, connection_hash: &str) -> Result<(), &'static str> {
        let index = self.connections.iter().position(|r| r == connection_hash);
        match index {
            None => { return Err("connection does not exist"); },
            Some(i) => { self.connections.remove(i); }
        }
        Ok(())
	}

	#[allow(dead_code)]
	pub fn list_connections(&self) {
        println!("{:#?}", self.connections);
	}

	pub fn filename(&self) -> String {
		format!("{}.todo", self.hash)
	}
	pub fn to_file_string(&self) -> String {
		let mut file_string: String = String::from("");
		file_string += &format!("[title]\n{}\n\n", self.title);
		file_string += &format!("[content]\n{}\n", self.content);
		file_string += &format!("[timestamp]\n{}\n\n", self.time);
		file_string += &format!("[tags]\n");
		for tag in &self.tags {
			file_string += &format!("{}\n", tag);
		}
		file_string += &format!("\n[connections]\n");
		for connection in &self.connections {
			file_string += &format!("{}\n", connection);
		}

		file_string
	}
}
