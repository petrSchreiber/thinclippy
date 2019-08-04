use crate::thinbasic_script::{Code, IssueSummary};

pub fn analysis_available(code: &mut Code) -> bool {
    let content = code.get_file_content().unwrap();

    content.contains("#FBCODE") || content.contains("#ENDFBCODE")
}

pub fn pairs_match(code: &mut Code) -> Result<(), IssueSummary> {
    let content = code.get_file_content().unwrap();

    let lines = content.lines();

    let mut opened_fb_code = false;
    let mut num_opened = 0;
    let mut num_closed = 0;
    let mut last_opened_fb_code_line = 0;

    let mut line_number = 0;

    for line in lines {
        let mut tokens = line.split_whitespace();
        let first_token_peek = tokens.next();
        let mut first_token = "";

        match first_token_peek {
            None => continue,
            Some(v) => first_token = v,
        };

        line_number += 1;

        if first_token == "#FBCODE" {
            if !opened_fb_code {
                opened_fb_code = true;
                num_opened += 1;
                last_opened_fb_code_line = line_number;
            } else {
                return Err(IssueSummary::new(
                    &code.main_file_name[..],
                    line_number,
                    1,
                    "Nested #FBCODE not supported",
                ));
            }
        }

        if first_token == "#ENDFBCODE" {
            if !opened_fb_code {
                return Err(IssueSummary::new(
                    &code.main_file_name[..],
                    line_number,
                    1,
                    "#ENDFBCODE without #FBCODE",
                ));
            } else {
                opened_fb_code = false;
                num_closed += 1;
            }
        }
    }

    if num_opened > num_closed {
        return Err(IssueSummary::new(
            &code.main_file_name[..],
            last_opened_fb_code_line,
            1,
            "#FBCODE does not have matching #ENDFBCODE",
        ));
    } else {
        Ok(())
    }
}
