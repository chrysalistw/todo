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
		todo!();
	}

	#[allow(dead_code)]
	pub fn set_time(&mut self) {
		todo!();
	}

	pub fn set_content(&mut self, content: &str){
		if self.content != "" {
			println!("content already exists: {}", self.content);
			return;
		}
		self.content = content.to_string();
	}

	#[allow(dead_code)]
	pub fn add_tag(&mut self) {
		todo!();
	}

	#[allow(dead_code)]
	pub fn remove_tag(&mut self) {
		todo!();
	}

	#[allow(dead_code)]
	pub fn list_tags(&mut self) {
		todo!();
	}

	#[allow(dead_code)]
	pub fn to_file_string(&self) -> String {
		todo!();
	}
}
