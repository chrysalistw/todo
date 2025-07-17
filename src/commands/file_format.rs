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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_file_creation() {
        let todo = TodoFile::new();
        assert_eq!(todo.hash, "");
        assert_eq!(todo.title, "");
        assert_eq!(todo.content, "");
        assert_eq!(todo.time, "");
        assert_eq!(todo.tags.len(), 0);
        assert_eq!(todo.connections.len(), 0);
    }

    #[test]
    fn test_set_title() {
        let mut todo = TodoFile::new();
        todo.set_title("Test Title");
        assert_eq!(todo.title, "Test Title");
        
        // Test that title cannot be overwritten
        todo.set_title("New Title");
        assert_eq!(todo.title, "Test Title");
    }

    #[test]
    fn test_set_content() {
        let mut todo = TodoFile::new();
        todo.set_content("Test content");
        assert_eq!(todo.content, "Test content");
        
        // Test that content cannot be overwritten
        todo.set_content("New content");
        assert_eq!(todo.content, "Test content");
    }

    #[test]
    fn test_set_time() {
        let mut todo = TodoFile::new();
        todo.set_time();
        assert_ne!(todo.time, "");
        assert!(todo.time.len() > 0);
    }

    #[test]
    fn test_generate_hash() {
        let mut todo = TodoFile::new();
        todo.set_content("Test content");
        todo.set_time();
        todo.generate_hash();
        assert_ne!(todo.hash, "");
        assert!(todo.hash.len() > 0);
    }

    #[test]
    fn test_hash_consistency() {
        let mut todo1 = TodoFile::new();
        todo1.set_content("Same content");
        todo1.time = "2023-01-01 12:00:00".to_string();
        todo1.generate_hash();
        
        let mut todo2 = TodoFile::new();
        todo2.set_content("Same content");
        todo2.time = "2023-01-01 12:00:00".to_string();
        todo2.generate_hash();
        
        assert_eq!(todo1.hash, todo2.hash);
    }

    #[test]
    fn test_filename() {
        let mut todo = TodoFile::new();
        todo.hash = "abcdef123456".to_string();
        assert_eq!(todo.filename(), "abcdef123456.todo");
    }

    #[test]
    fn test_add_tag() {
        let mut todo = TodoFile::new();
        
        // Test adding a tag
        assert!(todo.add_tag("work").is_ok());
        assert_eq!(todo.tags.len(), 1);
        assert_eq!(todo.tags[0], "work");
        
        // Test adding duplicate tag
        assert!(todo.add_tag("work").is_err());
        assert_eq!(todo.tags.len(), 1);
        
        // Test adding another tag
        assert!(todo.add_tag("urgent").is_ok());
        assert_eq!(todo.tags.len(), 2);
    }

    #[test]
    fn test_remove_tag() {
        let mut todo = TodoFile::new();
        todo.add_tag("work").unwrap();
        todo.add_tag("urgent").unwrap();
        
        // Test removing existing tag
        assert!(todo.remove_tag("work").is_ok());
        assert_eq!(todo.tags.len(), 1);
        assert_eq!(todo.tags[0], "urgent");
        
        // Test removing non-existent tag
        assert!(todo.remove_tag("nonexistent").is_err());
        assert_eq!(todo.tags.len(), 1);
    }

    #[test]
    fn test_add_connection() {
        let mut todo = TodoFile::new();
        
        // Test adding a connection
        assert!(todo.add_connection("abc123").is_ok());
        assert_eq!(todo.connections.len(), 1);
        assert_eq!(todo.connections[0], "abc123");
        
        // Test adding duplicate connection
        assert!(todo.add_connection("abc123").is_err());
        assert_eq!(todo.connections.len(), 1);
        
        // Test adding another connection
        assert!(todo.add_connection("def456").is_ok());
        assert_eq!(todo.connections.len(), 2);
    }

    #[test]
    fn test_remove_connection() {
        let mut todo = TodoFile::new();
        todo.add_connection("abc123").unwrap();
        todo.add_connection("def456").unwrap();
        
        // Test removing existing connection
        assert!(todo.remove_connection("abc123").is_ok());
        assert_eq!(todo.connections.len(), 1);
        assert_eq!(todo.connections[0], "def456");
        
        // Test removing non-existent connection
        assert!(todo.remove_connection("nonexistent").is_err());
        assert_eq!(todo.connections.len(), 1);
    }

    #[test]
    fn test_to_file_string() {
        let mut todo = TodoFile::new();
        todo.set_title("Test Title");
        todo.set_content("Test content");
        todo.time = "2023-01-01 12:00:00".to_string();
        todo.add_tag("work").unwrap();
        todo.add_tag("urgent").unwrap();
        todo.add_connection("abc123").unwrap();
        
        let file_string = todo.to_file_string();
        
        assert!(file_string.contains("[title]"));
        assert!(file_string.contains("Test Title"));
        assert!(file_string.contains("[content]"));
        assert!(file_string.contains("Test content"));
        assert!(file_string.contains("[timestamp]"));
        assert!(file_string.contains("2023-01-01 12:00:00"));
        assert!(file_string.contains("[tags]"));
        assert!(file_string.contains("work"));
        assert!(file_string.contains("urgent"));
        assert!(file_string.contains("[connections]"));
        assert!(file_string.contains("abc123"));
    }
}
