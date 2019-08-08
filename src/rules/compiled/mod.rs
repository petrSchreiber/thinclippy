use crate::thinbasic_script::{Code, IssueSummary};
use crate::tokenizer;
use crate::tokenizer::TokenType;

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

pub fn pairs_match(code: &mut Code) -> Vec<IssueSummary> {
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
                        //if parse_whitespace(&token_iter) {
                        let next_token = token_iter.peek();
                        if next_token.unwrap().token_type == TokenType::Whitespace {
                            token_iter.next();

                            let next_token = token_iter.peek();

                            // SUPPRESSRTE
                            if next_token.unwrap().token_type
                                == TokenType::Symbol("SUPPRESSRTE".to_string())
                            {
                                token_iter.next();

                                let next_token = token_iter.peek();
                                if next_token.unwrap().token_type == TokenType::Whitespace {
                                    token_iter.next();
                                }
                            }

                            // LANGUAGE
                            let next_token = token_iter.peek();
                            if next_token.unwrap().token_type
                                == TokenType::Symbol("LANGUAGE".to_string())
                            {
                                token_iter.next();

                                let next_token = token_iter.peek();
                                if next_token.unwrap().token_type == TokenType::Whitespace {
                                    token_iter.next();
                                }

                                let next_token = token_iter.peek();
                                if next_token.unwrap().token_type != TokenType::EqualSign {
                                    issues_found.push(IssueSummary::new(
                                        &code.main_file_name[..],
                                        next_token.unwrap().line,
                                        next_token.unwrap().pos,
                                        "#COMPILED LANGUAGE parameter must be followed by equal sign '='",
                                    ));
                                }

                                token_iter.next();
                                let next_token = token_iter.peek();
                                if next_token.unwrap().token_type == TokenType::Whitespace {
                                    token_iter.next();
                                }

                                let next_token = token_iter.peek();
                                if next_token.unwrap().token_type
                                    != TokenType::Symbol("FREEBASIC".to_string())
                                {
                                    issues_found.push(IssueSummary::new(
                                        &code.main_file_name[..],
                                        next_token.unwrap().line,
                                        next_token.unwrap().pos,
                                        "The only valid value for #COMPILED LANGUAGE parameter is FREEBASIC",
                                    ));
                                }
                            }
                        }
                    } else {
                        issues_found.push(IssueSummary::new(
                            &code.main_file_name[..],
                            token.line,
                            token.pos,
                            "Nested #COMPILED not supported",
                        ));
                    }
                }

                if kind == &end_compiled_str {
                    if !in_compiled_block {
                        issues_found.push(IssueSummary::new(
                            &code.main_file_name[..],
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
            &code.main_file_name[..],
            last_opened_compile_token_line,
            last_opened_compile_token_pos,
            "#COMPILED does not have matching #ENDCOMPILED",
        ));
    }

    issues_found
}
