use std::fs;

pub struct Code {
    pub main_file_name: String,

    file_content: String,
}

impl Code {
    pub fn new(main_file_name: String) -> Code {
        Code {
            main_file_name,
            file_content: "".to_string(),
        }
    }

    pub fn get_file_content(&mut self) -> Result<&String, &'static str> {
        if self.file_content.len() == 0 {
            match fs::read_to_string(&self.main_file_name) {
                Ok(v) => {
                    self.file_content = v.to_uppercase();
                    Ok(())
                }
                Err(_) => Err(format!("Could not read {} script.", &self.main_file_name)),
            };
        }

        Ok(&self.file_content)
    }
}
