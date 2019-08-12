use crate::thinbasic_script::{Code, IssueSummary};
use crate::tokenizer;
use crate::tokenizer::TokenType;

pub fn check(code: &mut Code) -> Vec<IssueSummary> {
    let mut issues_found: Vec<IssueSummary> = vec![];

    let tokens = code.get_tokens();
    let mut token_iter = tokens.iter().peekable();

    let alias_str = "ALIAS".to_string();

    let file_name = &code.main_file_name[..];

    let mut in_compiled = false;
    let compiled_str = "#COMPILED".to_string();
    let end_compiled_str = "#ENDCOMPILED".to_string();

    while let Some(&token) = token_iter.peek() {
        match &token.token_type {
            TokenType::Symbol(kind) => {
                token_iter.next();

                if kind == &compiled_str {
                    in_compiled = true;
                    token_iter.next();
                    continue;
                }

                if kind == &end_compiled_str {
                    in_compiled = false;
                    token_iter.next();
                    continue;
                }

                if kind == &alias_str {
                    if in_compiled {
                        token_iter.next();
                        continue;
                    }

                    if !tokenizer::parse_whitespace(&mut token_iter) {
                        issues_found.push(IssueSummary::new(
                            file_name,
                            token.line,
                            token.pos + 5,
                            "ALIAS keyword must be followed by whitespace",
                        ));

                        token_iter.next();
                        continue;
                    }

                    if !tokenizer::parse_any_symbol(&mut token_iter) {
                        if !tokenizer::parse_any_text(&mut token_iter) {
                            // This would indicate ALIAS in function
                            let next_token = token_iter.peek().unwrap();

                            issues_found.push(IssueSummary::new(
                                file_name,
                                next_token.line,
                                next_token.pos,
                                "There must be exactly one keyword specified after ALIAS",
                            ));
                        }

                        token_iter.next();
                        continue;
                    }

                    let next_token = token_iter.peek().unwrap();
                    let next_token_pos = next_token.pos - 1;

                    if !tokenizer::parse_whitespace(&mut token_iter) {
                        let caused_by_eol = tokenizer::parse_end_of_line(&mut token_iter);
                        let decr_line = if caused_by_eol { 2 } else { 0 };

                        let next_token = token_iter.peek().unwrap();

                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line - decr_line,
                            next_token_pos,
                            "Aliased keyword must be followed by whitespace",
                        ));

                        token_iter.next();
                        continue;
                    }

                    if !tokenizer::parse_symbol(&mut token_iter, "AS") {
                        let next_token = token_iter.peek().unwrap();
                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line,
                            next_token.pos,
                            "Single keyword after ALIAS must be followed by AS",
                        ));

                        token_iter.next();
                        continue;
                    }

                    if !tokenizer::parse_whitespace(&mut token_iter) {
                        let next_token = token_iter.peek().unwrap();
                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line,
                            next_token.pos,
                            "AS must be followed by whitespace",
                        ));

                        token_iter.next();
                        continue;
                    }

                    if !tokenizer::parse_any_symbol(&mut token_iter) {
                        let next_token = token_iter.peek().unwrap();
                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line,
                            next_token.pos,
                            "New name specified after AS must be a single word",
                        ));

                        token_iter.next();
                        continue;
                    }

                    if !tokenizer::is_last_on_line(&mut token_iter) {
                        let next_token = token_iter.peek().unwrap();
                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line,
                            next_token.pos,
                            "Only one word is allowed as alias after AS keyword",
                        ));

                        token_iter.next();
                        continue;
                    }
                }
            }

            _ => {
                token_iter.next();
            }
        }
    }

    issues_found
}
