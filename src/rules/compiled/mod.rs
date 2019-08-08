use crate::thinbasic_script::{Code, IssueSummary};
use crate::tokenizer;

pub fn analysis_available(code: &mut Code) -> bool {
    let tokens = code.get_tokens();

    for token in tokens {
        if token.token_type == tokenizer::TokenType::Symbol("#COMPILED".to_string()) {
            return true;
        }

        if token.token_type == tokenizer::TokenType::Symbol("#ENDCOMPILED".to_string()) {
            return true;
        }
    }

    false
}

pub fn pairs_match(code: &mut Code) -> Result<(), IssueSummary> {
    let tokens = code.get_tokens();

    let mut opened_fb_code = false;
    let mut num_opened = 0;
    let mut num_closed = 0;
    let mut last_opened_compile_token_line = 0;
    let mut last_opened_compile_token_pos = 0;

    for token in tokens {
        if token.token_type == tokenizer::TokenType::Symbol("#COMPILED".to_string()) {
            if !opened_fb_code {
                opened_fb_code = true;
                num_opened += 1;
                last_opened_compile_token_line = token.line;
                last_opened_compile_token_pos = token.pos;
            } else {
                return Err(IssueSummary::new(
                    &code.main_file_name[..],
                    token.line,
                    token.pos,
                    "Nested #COMPILED not supported",
                ));
            }
        }

        if token.token_type == tokenizer::TokenType::Symbol("#ENDCOMPILED".to_string()) {
            if !opened_fb_code {
                return Err(IssueSummary::new(
                    &code.main_file_name[..],
                    token.line,
                    token.pos,
                    "#ENDCOMPILE without #COMPILED",
                ));
            } else {
                opened_fb_code = false;
                num_closed += 1;
            }
        }
    }

    if num_opened > num_closed {
        Err(IssueSummary::new(
            &code.main_file_name[..],
            last_opened_compile_token_line,
            last_opened_compile_token_pos,
            "#COMPILED does not have matching #ENDCOMPILED",
        ))
    } else {
        Ok(())
    }
}
