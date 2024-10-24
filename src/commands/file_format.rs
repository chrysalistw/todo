use chrono;

#[derive(Debug)]
#[allow(dead_code)]
pub struct TodoFile {
		number: u32,
		time: String,
		content: String,
		tags: Vec<String>,
}

impl TodoFile {
	pub fn new() -> Self {
		Self {
			number: 0,
			time: String::from(""),
			content: String::from(""),
			tags: vec!(),
		}
	}

	#[allow(dead_code)]
	pub fn set_number(&mut self) {
        // how to get latest serial number of todo file?
		todo!();
	}

	#[allow(dead_code)]
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

	#[allow(dead_code)]
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

	#[allow(dead_code)]
	pub fn to_file_string(&self) -> String {
		todo!();
	}
}
