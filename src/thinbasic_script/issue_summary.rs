use std::fmt;

pub struct IssueSummary {
    script_file: String,

    line: u32,
    character: u32,

    summary: String,
}

impl IssueSummary {
    pub fn new(script_file: &str, line: u32, character: u32, summary: &str) -> IssueSummary {
        IssueSummary {
            script_file: script_file.to_string(),
            line,
            character,
            summary: summary.to_string(),
        }
    }
}

// Custom transformation to str, for text representation
impl fmt::Display for IssueSummary {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.summary.as_str())?;

        fmt.write_str(", line: ")?;
        fmt.write_str(&self.line.to_string()[..])?;

        fmt.write_str(", pos: ")?;
        fmt.write_str(&self.character.to_string()[..])?;

        fmt.write_str(" in ")?;
        fmt.write_str(self.script_file.as_str())?;

        Ok(())
    }
}
