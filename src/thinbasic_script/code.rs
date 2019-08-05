use std::fs;

pub struct Code {
    pub main_file_name: String,

    file_content: String,
}

impl Code {
    pub fn new(main_file_name: String) -> Result<Code, &'static str> {
        let file_content = match fs::read_to_string(&main_file_name) {
            Ok(content) => content.to_uppercase(),
            Err(_) => return Err("Could not load script file contents"),
        };

        Ok(Code {
            main_file_name,
            file_content,
        })
    }

    pub fn get_file_content(&mut self) -> Result<&String, &'static str> {
        if self.file_content.is_empty() {
            self.file_content = match fs::read_to_string(&self.main_file_name) {
                Ok(content) => content.to_uppercase(),
                Err(_) => return Err("Could not load script file contents"),
            }
        }

        Ok(&self.file_content)
    }
}
