use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub token_type: TokenType,
    pub line: u32,
    pub pos: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Whitespace,
    EndOfLine,
    Paren(char),
    EqualSign,
    Operator(char),
    Comparator(String),
    Number(String),
    Symbol(String),
    Text(String),
    Comment(String),
    Unknown(char),
    Comma,
}

fn get_number<T: Iterator<Item = char>>(char_iter: &mut Peekable<T>, pos: &mut u32) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            '0'...'9' | '.' => {
                char_iter.next();
                token.push(c);
                *pos += 1;
            }

            _ => return token,
        }
    }
    token
}

fn get_comparator<T: Iterator<Item = char>>(char_iter: &mut Peekable<T>, pos: &mut u32) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            '<' | '>' | '=' => {
                char_iter.next();
                token.push(c);
                *pos += 1;
            }

            _ => return token,
        }
    }
    token
}

fn get_whitespace<T: Iterator<Item = char>>(char_iter: &mut Peekable<T>, pos: &mut u32) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            ' ' | '\t' => {
                char_iter.next();
                token.push(c);
                *pos += 1;
            }

            _ => return token,
        }
    }
    token
}

fn get_symbol<T: Iterator<Item = char>>(char_iter: &mut Peekable<T>, pos: &mut u32) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            'A'...'Z' | '#' | '$' | '%' | '_' => {
                char_iter.next();
                token.push(c);
                *pos += 1;
            }

            'a'...'z' => {
                char_iter.next();
                token.push(c.to_string().to_uppercase().chars().next().unwrap());
                *pos += 1;
            }

            _ => return token,
        }
    }

    token
}

fn get_single_comment<T: Iterator<Item = char>>(
    char_iter: &mut Peekable<T>,
    pos: &mut u32,
) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            '\n' => return token,

            '\r' => {
                char_iter.next();
            }

            _ => {
                char_iter.next();
                token.push(c);
                *pos += 1;
            }
        }
    }

    token
}

fn get_block_comment<T: Iterator<Item = char>>(
    char_iter: &mut Peekable<T>,
    line_no: &mut u32,
    pos: &mut u32,
) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            '*' => {
                char_iter.next();
                *pos += 1;

                if char_iter.peek() == Some(&'/') {
                    // Detecting ending */
                    char_iter.next();
                    *pos += 1;

                    return token;
                } else {
                    *pos += 1;
                    token.push(c);
                }
            }

            '\r' => {
                char_iter.next();
            }

            '\n' => {
                char_iter.next();
                *line_no += 1;
                *pos = 0;
                token.push(c);
            }

            _ => {
                char_iter.next();
                *pos += 1;
                token.push(c);
            }
        }
    }

    token
}

fn get_text<T: Iterator<Item = char>>(
    char_iter: &mut Peekable<T>,
    line_no: &mut u32,
    pos: &mut u32,
) -> String {
    let mut token = String::new();
    let mut quote_level = 0;

    while let Some(&c) = char_iter.peek() {
        match c {
            '\"' => {
                quote_level += 1;

                char_iter.next();
                *pos += 1;
                token.push(c);

                if quote_level == 2 {
                    if char_iter.peek() == Some(&'\"') {
                        char_iter.next();
                        *pos += 1;
                        token.push(c);
                        quote_level -= 1;
                    } else {
                        return token;
                    }
                }
            }

            '\r' => {
                char_iter.next();
            }

            '\n' => {
                char_iter.next();
                *line_no += 1;
                *pos = 0;
                token.push(c);
            }

            _ => {
                char_iter.next();
                *pos += 1;
                token.push(c);
            }
        }
    }

    token
}

pub fn get_tokens(input: &str) -> Vec<TokenInfo> {
    let mut simple_tokens: Vec<TokenInfo> = vec![];

    let mut char_iter = input.chars().peekable();

    let mut line_no = 1;
    let mut pos_no = 0;
    let mut start_pos_no;

    while let Some(&c) = char_iter.peek() {
        start_pos_no = pos_no + 1;
        match c {
            'A'...'Z' | 'a'...'z' | '#' | '$' | '%' | '_' => {
                let symbol = get_symbol(&mut char_iter, &mut pos_no);

                if symbol == "REM" {
                    let comment_token = get_single_comment(&mut char_iter, &mut pos_no);
                    simple_tokens.push(TokenInfo {
                        token_type: TokenType::Comment(comment_token),
                        line: line_no,
                        pos: start_pos_no,
                    });
                } else {
                    simple_tokens.push(TokenInfo {
                        token_type: TokenType::Symbol(symbol),
                        line: line_no,
                        pos: start_pos_no,
                    });
                }
            }

            '0'...'9' | '.' => {
                let number_token = get_number(&mut char_iter, &mut pos_no);
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Number(number_token),
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            '<' | '>' => {
                let comparator_token = get_comparator(&mut char_iter, &mut pos_no);
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Comparator(comparator_token),
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            '/' => {
                char_iter.next(); // Absorb first /

                if char_iter.peek() == Some(&'/') {
                    char_iter.next(); // Absorb second /

                    let comment_token = get_single_comment(&mut char_iter, &mut pos_no);
                    simple_tokens.push(TokenInfo {
                        token_type: TokenType::Comment(comment_token),
                        line: line_no,
                        pos: start_pos_no,
                    });
                } else if char_iter.peek() == Some(&'*') {
                    char_iter.next(); // Absorb *

                    let comment_token =
                        get_block_comment(&mut char_iter, &mut line_no, &mut pos_no);
                    simple_tokens.push(TokenInfo {
                        token_type: TokenType::Comment(comment_token),
                        line: line_no,
                        pos: start_pos_no,
                    });
                } else {
                    simple_tokens.push(TokenInfo {
                        token_type: TokenType::Operator(c),
                        line: line_no,
                        pos: start_pos_no,
                    });
                }
            }

            '+' | '*' | '-' | '&' => {
                char_iter.next();
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Operator(c),
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            ' ' | '\t' => {
                let _whitespace = get_whitespace(&mut char_iter, &mut pos_no);
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Whitespace,
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            '(' | ')' => {
                char_iter.next();
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Paren(c),
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            '\'' => {
                char_iter.next(); // Absorb '

                let comment_token = get_single_comment(&mut char_iter, &mut pos_no);
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Comment(comment_token),
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            '"' => {
                let text_string = get_text(&mut char_iter, &mut line_no, &mut pos_no);
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Text(text_string),
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            '\r' => {
                char_iter.next();
            }

            '\n' => {
                char_iter.next();
                line_no += 1;
                pos_no = 0;
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::EndOfLine,
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            '=' => {
                char_iter.next();
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::EqualSign,
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            ',' => {
                char_iter.next();
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Comma,
                    line: line_no,
                    pos: start_pos_no,
                });
            }

            '\u{feff}' => {
                // Just BOM
                char_iter.next();
                pos_no += 1;
            }

            _ => {
                char_iter.next();
                simple_tokens.push(TokenInfo {
                    token_type: TokenType::Unknown(c),
                    line: line_no,
                    pos: start_pos_no,
                });
            }
        }
    }

    simple_tokens
}

#[cfg(test)]
pub mod tests {

    use super::get_tokens;
    use super::TokenType;

    #[test]
    fn get_number_works() {
        let code = "1234";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(0).unwrap().token_type,
            TokenType::Number("1234".to_string())
        );
    }

    #[test]
    fn get_symbol_works() {
        let code = "%Ciao_my_FRIEND$";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(0).unwrap().token_type,
            TokenType::Symbol("%CIAO_MY_FRIEND$".to_string())
        );
    }

    #[test]
    fn get_operator_works() {
        let code = "a + b - c * d / e & f";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(2).unwrap().token_type, TokenType::Operator('+'));
        assert_eq!(tokens.get(6).unwrap().token_type, TokenType::Operator('-'));
        assert_eq!(tokens.get(10).unwrap().token_type, TokenType::Operator('*'));
        assert_eq!(tokens.get(14).unwrap().token_type, TokenType::Operator('/'));
        assert_eq!(tokens.get(18).unwrap().token_type, TokenType::Operator('&'));
    }

    #[test]
    fn get_whitespace_works() {
        let code = "A    \tB";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(1).unwrap().token_type, TokenType::Whitespace);
    }

    #[test]
    fn get_paren_works() {
        let code = "(1)";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(0).unwrap().token_type, TokenType::Paren('('));
        assert_eq!(tokens.get(2).unwrap().token_type, TokenType::Paren(')'));
    }

    #[test]
    fn get_comma_works() {
        let code = "a,b,c";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(1).unwrap().token_type, TokenType::Comma);
        assert_eq!(tokens.get(3).unwrap().token_type, TokenType::Comma);
    }

    #[test]
    fn get_apostrophe_comment() {
        let code = "hello world ' famous sentence\nindeed";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(4).unwrap().token_type,
            TokenType::Comment(" famous sentence".to_string())
        );
        assert_eq!(tokens.get(5).unwrap().token_type, TokenType::EndOfLine);
        assert_eq!(
            tokens.get(6).unwrap().token_type,
            TokenType::Symbol("INDEED".to_string())
        );
    }

    #[test]
    fn get_c_comment() {
        let code = "hello world // famous sentence\nindeed";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(4).unwrap().token_type,
            TokenType::Comment(" famous sentence".to_string())
        );
        assert_eq!(tokens.get(5).unwrap().token_type, TokenType::EndOfLine);
        assert_eq!(
            tokens.get(6).unwrap().token_type,
            TokenType::Symbol("INDEED".to_string())
        );
    }

    #[test]
    fn get_block_comment() {
        let code = "hello /* famous \n sentence */ indeed";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(2).unwrap().token_type,
            TokenType::Comment(" famous \n sentence ".to_string())
        );
        assert_eq!(tokens.get(3).unwrap().token_type, TokenType::Whitespace);
        assert_eq!(
            tokens.get(4).unwrap().token_type,
            TokenType::Symbol("INDEED".to_string())
        );
    }

    #[test]
    fn get_rem_comment() {
        let code = "hello REM famous \nindeed";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(2).unwrap().token_type,
            TokenType::Comment(" famous ".to_string())
        );
        assert_eq!(
            tokens.get(4).unwrap().token_type,
            TokenType::Symbol("INDEED".to_string())
        );
    }

    #[test]
    fn get_end_of_line() {
        let code = "A\nB\r\nC";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(1).unwrap().token_type, TokenType::EndOfLine);
        assert_eq!(tokens.get(3).unwrap().token_type, TokenType::EndOfLine);
    }

    #[test]
    fn get_text() {
        let code = "hello = \"Ciao\"";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(4).unwrap().token_type,
            TokenType::Text("\"Ciao\"".to_string())
        );
    }

    #[test]
    fn get_text_string_with_inner_quotes() {
        let code = "hello = \"Ciao, dear \"\"bambino\"\"\"";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(4).unwrap().token_type,
            TokenType::Text("\"Ciao, dear \"\"bambino\"\"\"".to_string())
        );
    }

    #[test]
    fn get_equal_sign() {
        let code = "numero = 1";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(2).unwrap().token_type, TokenType::EqualSign);
    }

    #[test]
    fn get_comparator() {
        let code = "a>=b,c<=d,e<f,g>h";

        let tokens = get_tokens(code);

        assert_eq!(
            tokens.get(1).unwrap().token_type,
            TokenType::Comparator(">=".to_string())
        );
        assert_eq!(
            tokens.get(5).unwrap().token_type,
            TokenType::Comparator("<=".to_string())
        );
        assert_eq!(
            tokens.get(9).unwrap().token_type,
            TokenType::Comparator("<".to_string())
        );
        assert_eq!(
            tokens.get(13).unwrap().token_type,
            TokenType::Comparator(">".to_string())
        );
    }

    #[test]
    fn get_unknown() {
        let code = "this is ~";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(4).unwrap().token_type, TokenType::Unknown('~'));
    }

    #[test]
    fn line_location() {
        let code = "a\r\nb\nc";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(0).unwrap().line, 1u32);
        assert_eq!(tokens.get(2).unwrap().line, 2u32);
        assert_eq!(tokens.get(4).unwrap().line, 3u32);
    }

    #[test]
    fn pos_location() {
        let code = " well\r\n  this\n   works";

        let tokens = get_tokens(code);

        assert_eq!(tokens.get(0).unwrap().token_type, TokenType::Whitespace);
        assert_eq!(
            tokens.get(1).unwrap().token_type,
            TokenType::Symbol("WELL".to_string())
        );
        assert_eq!(tokens.get(2).unwrap().token_type, TokenType::EndOfLine);
        assert_eq!(tokens.get(3).unwrap().token_type, TokenType::Whitespace);
        assert_eq!(
            tokens.get(4).unwrap().token_type,
            TokenType::Symbol("THIS".to_string())
        );
        assert_eq!(tokens.get(5).unwrap().token_type, TokenType::EndOfLine);
        assert_eq!(tokens.get(6).unwrap().token_type, TokenType::Whitespace);
        assert_eq!(
            tokens.get(7).unwrap().token_type,
            TokenType::Symbol("WORKS".to_string())
        );

        assert_eq!(tokens.get(1).unwrap().pos, 2u32);
        assert_eq!(tokens.get(4).unwrap().pos, 3u32);
        assert_eq!(tokens.get(7).unwrap().pos, 4u32);
    }

}
