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

fn get_number<T: Iterator<Item = char>>(c: char, character_iterator: &mut Peekable<T>) -> String {
    let mut token = String::new();

    while let Some(&c) = character_iterator.peek() {
        match c {
            '0'...'9' | '.' => {
                character_iterator.next();
                token.push(c);
            }

            _ => return token,
        }
    }

    return token;
}

fn get_symbol<T: Iterator<Item = char>>(c: char, character_iterator: &mut Peekable<T>) -> String {
    let mut token = String::new();

    while let Some(&c) = character_iterator.peek() {
        match c {
            'A'...'Z' | '#' | '$' | '%' | '_' => {
                character_iterator.next();
                token.push(c);
            }

            'a'...'z' => {
                character_iterator.next();
                token.push(c.to_string().to_uppercase().chars().next().unwrap());
            }

            _ => return token,
        }
    }

    return token;
}

fn get_comment<T: Iterator<Item = char>>(c: char, character_iterator: &mut Peekable<T>) -> String {
    let mut token = String::new();

    while let Some(&c) = character_iterator.peek() {
        match c {
            '\'' => {
                character_iterator.next();
                token.push(c);
            }

            '\r' | '\n' => return token,

            _ => {
                character_iterator.next();
                token.push(c);
            }
        }
    }

    return token;
}

fn get_text_string<T: Iterator<Item = char>>(
    c: char,
    character_iterator: &mut Peekable<T>,
) -> String {
    let mut token = String::new();
    let mut quote_level = 0;
    while let Some(&c) = character_iterator.peek() {
        match c {
            '\"' => {
                quote_level += 1;

                if quote_level == 2 {
                    return token;
                }

                character_iterator.next();
                token.push(c);
            }

            _ => {
                character_iterator.next();
                token.push(c);
            }
        }
    }

    return token;
}

pub fn get_simple_tokens<'a>(input: &str) -> Vec<Token> {
    let mut simple_tokens: Vec<Token> = vec![];
    let mut token = String::new();

    let mut character_iterator = input.chars().peekable();

    while let Some(&c) = character_iterator.peek() {
        match c {
            'A'...'Z' | 'a'...'z' | '#' | '$' | '%' | '_' => {
                let text_item = get_symbol(c, &mut character_iterator);
                simple_tokens.push(Token::Symbol(text_item));
            }

            '0'...'9' | '.' => {
                let number_token = get_number(c, &mut character_iterator);
                simple_tokens.push(Token::Number(number_token));
            }

            '+' | '*' | '/' | '-' => {
                character_iterator.next();
                simple_tokens.push(Token::Operator(c))
            }

            ' ' | '\t' => {
                character_iterator.next();
                simple_tokens.push(Token::Whitespace)
            }

            '(' | ')' => {
                character_iterator.next();
                simple_tokens.push(Token::Paren(c))
            }

            '\'' => {
                let comment_token = get_comment(c, &mut character_iterator);
                simple_tokens.push(Token::Comment(comment_token));
            }

            '"' => {
                let text_string = get_comment(c, &mut character_iterator);
                simple_tokens.push(Token::TextString(text_string));
            }

            '\r' => {
                character_iterator.next();
                simple_tokens.push(Token::EndOfLine);

                // CRLF taken as one
                if character_iterator.peek() == Some(&'\n') {
                    character_iterator.next();
                }
            }

            '\n' => {
                character_iterator.next();
                simple_tokens.push(Token::EndOfLine)
            }

            '=' => {
                character_iterator.next();
                simple_tokens.push(Token::EqualSign)
            }

            _ => {
                character_iterator.next();
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
        let code = "+-*/";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(0), Some(&Token::Operator('+')));
        assert_eq!(tokens.get(1), Some(&Token::Operator('-')));
        assert_eq!(tokens.get(2), Some(&Token::Operator('*')));
        assert_eq!(tokens.get(3), Some(&Token::Operator('/')));
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
    fn get_comment() {
        let code = "hello world ' famous sentence\nindeed";

        let tokens = get_simple_tokens(code);

        assert_eq!(
            tokens.get(4),
            Some(&Token::Comment("' famous sentence".to_string()))
        );
        assert_eq!(tokens.get(5), Some(&Token::EndOfLine));
        assert_eq!(tokens.get(6), Some(&Token::Symbol("INDEED".to_string())));
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
    fn get_equal_sign() {
        let code = "numero = 1";

        let tokens = get_simple_tokens(code);

        assert_eq!(tokens.get(2), Some(&Token::EqualSign));
    }

}
