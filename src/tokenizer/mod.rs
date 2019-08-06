use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Whitespace,
    EndOfLine,
    Paren(char),
    EqualSign,
    Operator(char),
    Number(String),
    Symbol(String),
    TextString(String),
    Comment(String),
    Unknown(char),
}

fn get_number<T: Iterator<Item = char>>(c: char, char_iter: &mut Peekable<T>) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            '0'...'9' | '.' => {
                char_iter.next();
                token.push(c);
            }

            _ => return token,
        }
    }

    return token;
}

fn get_symbol<T: Iterator<Item = char>>(c: char, char_iter: &mut Peekable<T>) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            'A'...'Z' | '#' | '$' | '%' | '_' => {
                char_iter.next();
                token.push(c);
            }

            'a'...'z' => {
                char_iter.next();
                token.push(c.to_string().to_uppercase().chars().next().unwrap());
            }

            _ => return token,
        }
    }

    return token;
}

fn get_single_comment<T: Iterator<Item = char>>(c: char, char_iter: &mut Peekable<T>) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            '\r' | '\n' => return token,

            _ => {
                char_iter.next();
                token.push(c);
            }
        }
    }

    return token;
}

fn get_block_comment<T: Iterator<Item = char>>(c: char, char_iter: &mut Peekable<T>) -> String {
    let mut token = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            '*' => {
                char_iter.next();

                if char_iter.peek() == Some(&'/') {
                    // Detecting ending */
                    char_iter.next();
                    return token;
                } else {
                    token.push(c);
                }
            }

            _ => {
                char_iter.next();
                token.push(c);
            }
        }
    }

    return token;
}

fn get_text_string<T: Iterator<Item = char>>(c: char, char_iter: &mut Peekable<T>) -> String {
    let mut token = String::new();
    let mut quote_level = 0;

    while let Some(&c) = char_iter.peek() {
        match c {
            '\"' => {
                quote_level += 1;

                char_iter.next();
                token.push(c);

                if quote_level == 2 {
                    if char_iter.peek() == Some(&'\"') {
                        char_iter.next();
                        token.push(c);
                        quote_level -= 1;
                    } else {
                        return token;
                    }
                }
            }

            _ => {
                char_iter.next();
                token.push(c);
            }
        }
    }

    return token;
}

pub fn get_simple_tokens<'a>(input: &str) -> Vec<Token> {
    let mut simple_tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();

    while let Some(&c) = char_iter.peek() {
        match c {
            'A'...'Z' | 'a'...'z' | '#' | '$' | '%' | '_' => {
                let symbol = get_symbol(c, &mut char_iter);

                if symbol == "REM" {
                    let comment_token = get_single_comment(c, &mut char_iter);
                    simple_tokens.push(Token::Comment(comment_token));
                } else {
                    simple_tokens.push(Token::Symbol(symbol));
                }
            }

            '0'...'9' | '.' => {
                let number_token = get_number(c, &mut char_iter);
                simple_tokens.push(Token::Number(number_token));
            }

            '/' => {
                char_iter.next(); // Absorb first /

                if char_iter.peek() == Some(&'/') {
                    char_iter.next(); // Absorb second /

                    let comment_token = get_single_comment(c, &mut char_iter);
                    simple_tokens.push(Token::Comment(comment_token));
                } else {
                    if char_iter.peek() == Some(&'*') {
                        char_iter.next(); // Absorb *

                        let comment_token = get_block_comment(c, &mut char_iter);
                        simple_tokens.push(Token::Comment(comment_token));
                    } else {
                        simple_tokens.push(Token::Operator(c))
                    }
                }
            }

            '+' | '*' | '-' => {
                char_iter.next();
                simple_tokens.push(Token::Operator(c))
            }

            ' ' | '\t' => {
                char_iter.next();
                simple_tokens.push(Token::Whitespace)
            }

            '(' | ')' => {
                char_iter.next();
                simple_tokens.push(Token::Paren(c))
            }

            '\'' => {
                char_iter.next(); // Absorb '

                let comment_token = get_single_comment(c, &mut char_iter);
                simple_tokens.push(Token::Comment(comment_token));
            }

            '"' => {
                let text_string = get_text_string(c, &mut char_iter);
                simple_tokens.push(Token::TextString(text_string));
            }

            '\r' => {
                char_iter.next();
                simple_tokens.push(Token::EndOfLine);

                // CRLF taken as one
                if char_iter.peek() == Some(&'\n') {
                    char_iter.next();
                }
            }

            '\n' => {
                char_iter.next();
                simple_tokens.push(Token::EndOfLine)
            }

            '=' => {
                char_iter.next();
                simple_tokens.push(Token::EqualSign)
            }

            _ => {
                char_iter.next();
                simple_tokens.push(Token::Unknown(c))
            }
        }
    }

    return simple_tokens;
}

#[cfg(test)]
pub mod tests {

    use super::get_simple_tokens;
    use super::Token;

    #[test]
    fn get_number_works() {
        let code = "1234";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(0), Some(&Token::Number("1234".to_string())));
    }

    #[test]
    fn get_symbol_works() {
        let code = "%Ciao_my_FRIEND$";

        let tokens = get_simple_tokens(code);

        assert_eq!(
            tokens.get(0),
            Some(&Token::Symbol("%CIAO_MY_FRIEND$".to_string()))
        );
    }

    #[test]
    fn get_operator_works() {
        let code = "a + b - c * d / e";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(2), Some(&Token::Operator('+')));
        assert_eq!(tokens.get(6), Some(&Token::Operator('-')));
        assert_eq!(tokens.get(10), Some(&Token::Operator('*')));
        assert_eq!(tokens.get(14), Some(&Token::Operator('/')));
    }

    #[test]
    fn get_whitespace_works() {
        let code = "A    \tB";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(1), Some(&Token::Whitespace));
    }

    #[test]
    fn get_paren_works() {
        let code = "(1)";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(0), Some(&Token::Paren('(')));
        assert_eq!(tokens.get(2), Some(&Token::Paren(')')));
    }

    #[test]
    fn get_apostrophe_comment() {
        let code = "hello world ' famous sentence\nindeed";

        let tokens = get_simple_tokens(code);

        assert_eq!(
            tokens.get(4),
            Some(&Token::Comment(" famous sentence".to_string()))
        );
        assert_eq!(tokens.get(5), Some(&Token::EndOfLine));
        assert_eq!(tokens.get(6), Some(&Token::Symbol("INDEED".to_string())));
    }

    #[test]
    fn get_c_comment() {
        let code = "hello world // famous sentence\nindeed";

        let tokens = get_simple_tokens(code);

        assert_eq!(
            tokens.get(4),
            Some(&Token::Comment(" famous sentence".to_string()))
        );
        assert_eq!(tokens.get(5), Some(&Token::EndOfLine));
        assert_eq!(tokens.get(6), Some(&Token::Symbol("INDEED".to_string())));
    }

    #[test]
    fn get_block_comment() {
        let code = "hello /* famous \n sentence */ indeed";

        let tokens = get_simple_tokens(code);

        assert_eq!(
            tokens.get(2),
            Some(&Token::Comment(" famous \n sentence ".to_string()))
        );
        assert_eq!(tokens.get(3), Some(&Token::Whitespace));
        assert_eq!(tokens.get(4), Some(&Token::Symbol("INDEED".to_string())));
    }

    #[test]
    fn get_rem_comment() {
        let code = "hello REM famous \nindeed";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(2), Some(&Token::Comment(" famous ".to_string())));
        assert_eq!(tokens.get(4), Some(&Token::Symbol("INDEED".to_string())));
    }

    #[test]
    fn get_end_of_line() {
        let code = "A\nB\r\nC";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(1), Some(&Token::EndOfLine));
        assert_eq!(tokens.get(3), Some(&Token::EndOfLine));
    }

    #[test]
    fn get_text_string() {
        let code = "hello = \"Ciao\"";

        let tokens = get_simple_tokens(code);

        assert_eq!(
            tokens.get(4),
            Some(&Token::TextString("\"Ciao\"".to_string()))
        );
    }

    #[test]
    fn get_text_string_with_inner_quotes() {
        let code = "hello = \"Ciao, dear \"\"bambino\"\"\"";

        let tokens = get_simple_tokens(code);

        assert_eq!(
            tokens.get(4),
            Some(&Token::TextString(
                "\"Ciao, dear \"\"bambino\"\"\"".to_string()
            ))
        );
    }

    #[test]
    fn get_equal_sign() {
        let code = "numero = 1";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(2), Some(&Token::EqualSign));
    }

}
