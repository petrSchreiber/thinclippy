use crate::thinbasic_script::{Code, IssueSummary};
use crate::tokenizer;
use crate::tokenizer::TokenType;

pub fn section_definition(code: &mut Code) -> Vec<IssueSummary> {
    let mut issues_found: Vec<IssueSummary> = vec![];

    let tokens = code.get_tokens();
    let mut token_iter = tokens.iter().peekable();

    let mut in_compiled_block = false;

    let mut num_opened = 0;
    let mut num_closed = 0;

    let mut last_opened_compile_token_line = 0;
    let mut last_opened_compile_token_pos = 0;

    let compiled_str = "#COMPILED".to_string();
    let end_compiled_str = "#ENDCOMPILED".to_string();

    let file_name = &code.main_file_name[..];

    while let Some(&token) = token_iter.peek() {
        match &token.token_type {
            TokenType::Symbol(kind) => {
                token_iter.next();

                if kind == &compiled_str {
                    // Nesting check
                    if !in_compiled_block {
                        in_compiled_block = true;
                        num_opened += 1;

                        last_opened_compile_token_line = token.line;
                        last_opened_compile_token_pos = token.pos;

                        // Looking for parameters
                        if tokenizer::parse_whitespace(&mut token_iter) {
                            if tokenizer::parse_symbol(&mut token_iter, "SUPPRESSRTE") {
                                tokenizer::parse_whitespace(&mut token_iter);
                            }

                            let lang_token = token_iter.peek().unwrap();
                            let language_line = lang_token.line;
                            let language_end_pos = lang_token.pos + 8;

                            if tokenizer::parse_symbol(&mut token_iter, "LANGUAGE") {
                                tokenizer::parse_whitespace(&mut token_iter);
                                if !tokenizer::parse_equal_sign(&mut token_iter) {
                                    issues_found.push(IssueSummary::new(
                                        file_name,
                                        language_line,
                                        language_end_pos,
                                        "#COMPILED LANGUAGE parameter must be followed by equal sign '='",
                                    ));
                                } else {
                                    tokenizer::parse_whitespace(&mut token_iter);

                                    if !tokenizer::parse_symbol(&mut token_iter, "FREEBASIC") {
                                        let next_token = token_iter.peek().unwrap();
                                        issues_found.push(IssueSummary::new(
                                            file_name,
                                            next_token.line,
                                            next_token.pos,
                                            "The only valid value for #COMPILED LANGUAGE parameter is FREEBASIC",
                                        ));
                                    }
                                }
                            }
                        }
                    } else {
                        issues_found.push(IssueSummary::new(
                            file_name,
                            token.line,
                            token.pos,
                            "Nested #COMPILED not supported",
                        ));
                    }
                }

                if kind == &end_compiled_str {
                    if !in_compiled_block {
                        issues_found.push(IssueSummary::new(
                            file_name,
                            token.line,
                            token.pos,
                            "#ENDCOMPILE without #COMPILED",
                        ));
                    } else {
                        in_compiled_block = false;
                        num_closed += 1;
                    }
                }
            }

            _ => {
                token_iter.next();
            }
        }
    }

    if num_opened > num_closed {
        issues_found.push(IssueSummary::new(
            file_name,
            last_opened_compile_token_line,
            last_opened_compile_token_pos,
            "#COMPILED does not have matching #ENDCOMPILED",
        ));
    }

    issues_found
}
