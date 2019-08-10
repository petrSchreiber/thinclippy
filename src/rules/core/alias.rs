use crate::thinbasic_script::{Code, IssueSummary};
use crate::tokenizer;
use crate::tokenizer::TokenType;

pub fn check(code: &mut Code) -> Vec<IssueSummary> {
    let mut issues_found: Vec<IssueSummary> = vec![];

    let tokens = code.get_tokens();
    let mut token_iter = tokens.iter().peekable();

    let alias_str = "ALIAS".to_string();

    let file_name = &code.main_file_name[..];

    while let Some(&token) = token_iter.peek() {
        match &token.token_type {
            TokenType::Symbol(kind) => {
                token_iter.next();

                if kind == &alias_str {
                    if !tokenizer::parse_whitespace_and_any_symbol(&mut token_iter) {
                        let next_token = token_iter.peek().unwrap();
                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line,
                            next_token.pos,
                            "There must be exactly one keyword specified after ALIAS",
                        ));
                    } else if !tokenizer::parse_whitespace_and_symbol(&mut token_iter, "AS") {
                        let next_token = token_iter.peek().unwrap();
                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line,
                            next_token.pos,
                            "Single keyword after ALIAS must be followed by AS",
                        ));
                    } else if !tokenizer::parse_whitespace_and_any_symbol(&mut token_iter) {
                        let next_token = token_iter.peek().unwrap();
                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line,
                            next_token.pos,
                            "New name specified after AS must be a single word",
                        ));
                    } else if !tokenizer::is_last_on_line(&mut token_iter) {
                        let next_token = token_iter.peek().unwrap();
                        issues_found.push(IssueSummary::new(
                            file_name,
                            next_token.line,
                            next_token.pos,
                            "Only one word is allowed as alias after AS keyword",
                        ));
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
