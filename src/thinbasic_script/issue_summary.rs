use std::fmt;

pub struct IssueSummary {
    pub script_file: String,

    pub line: u32,
    pub pos: u32,

    pub summary: String,
}

impl IssueSummary {
    pub fn new(script_file: &str, line: u32, pos: u32, summary: &str) -> IssueSummary {
        IssueSummary {
            script_file: script_file.to_string(),
            line,
            pos,
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
        fmt.write_str(&self.pos.to_string()[..])?;

        fmt.write_str(" in ")?;
        fmt.write_str(self.script_file.as_str())?;

        Ok(())
    }
}
