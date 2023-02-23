#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    WhiteSpace,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
}

/// parse json string and divide it to tokens.
pub struct Lexer<'a> {
    /// point head character
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

/// error during lexical analysis
#[derive(Debug)]
pub struct LexerError {
    // error message
    pub msg: String,
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            msg: msg.to_string(),
        }
    }
}

impl<'a> Lexer<'a> {
    /// take string and returen Lexer
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    /// divide string to tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token()? {
            match token {
                Token::WhiteSpace => {}
                _ => {
                    tokens.push(token);
                }
            }
        }

        Ok(tokens)
    }

    /// advance a character and retun token
    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    /// take string and return matched Token
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        match self.chars.peek() {
            Some(c) => match c {
                c if c.is_whitespace() || *c == '\n' => {
                    Ok(self.next_return_token(Token::WhiteSpace))
                }
                '{' => Ok(self.next_return_token(Token::LeftBrace)),
                '}' => Ok(self.next_return_token(Token::RightBrace)),
                '[' => Ok(self.next_return_token(Token::LeftBracket)),
                ']' => Ok(self.next_return_token(Token::RightBracket)),
                ',' => Ok(self.next_return_token(Token::Comma)),
                ':' => Ok(self.next_return_token(Token::Colon)),

                // String
                '"' => {
                    self.chars.next();
                    self.parse_string_token()
                }

                // Number
                c if c.is_numeric() || matches!(c, '+' | '-' | '.') => self.parse_number_token(),

                // Boolean
                't' => self.parse_bool_token(true),
                'f' => self.parse_bool_token(false),

                // Null
                'n' => self.parse_null_token(),

                // error
                _ => Err(LexerError::new(&format!("error: an unexpected char {}", c))),
            },
            None => Ok(None),
        }
    }

    fn parse_null_token(&mut self) -> Result<Option<Token>, LexerError> {
        let s: String = (0..4).filter_map(|_| self.chars.next()).collect();
        if s == "null" {
            Ok(Some(Token::Null))
        } else {
            Err(LexerError::new(&format!(
                "error: a null value is expected {}",
                s
            )))
        }
    }

    /// parse (true|false) string
    fn parse_bool_token(&mut self, b: bool) -> Result<Option<Token>, LexerError> {
        if b {
            let s: String = (0..4).filter_map(|_| self.chars.next()).collect();
            if s == "true" {
                Ok(Some(Token::Bool(true)))
            } else {
                Err(LexerError::new(&format!(
                    "error: a boolean true value is expected {}",
                    s
                )))
            }
        } else {
            let s: String = (0..5).filter_map(|_| self.chars.next()).collect();
            if s == "false" {
                Ok(Some(Token::Bool(false)))
            } else {
                Err(LexerError::new(&format!(
                    "error: a boolean false value is expected {}",
                    s
                )))
            }
        }
    }
    /// parse number string
    fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut number_str = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() | matches!(c, '+' | '-' | 'e' | 'E' | '.') {
                self.chars.next();
                number_str.push(c);
            } else {
                break;
            }
        }
        match number_str.parse::<f64>() {
            Ok(number) => Ok(Some(Token::Number(number))),
            Err(e) => Err(LexerError::new(&format!("error: {}", e.to_string()))),
        }
    }
    /// read string until terminal character '/"'
    fn parse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }
    /// concatenate ***s if utf16 buffer exists
    fn push_utf16(result: &mut String, utf16: &mut Vec<u16>) -> Result<(), LexerError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let null = "null";
        let tokens = Lexer::new(null).tokenize().unwrap();
        assert_eq!(Token::Null, tokens[0]);
    }
    #[test]
    fn test_bool() {
        let b = "true";
        let tokens = Lexer::new(b).tokenize().unwrap();
        assert_eq!(Token::Bool(true), tokens[0]);
        let b = "false";
        let tokens = Lexer::new(b).tokenize().unwrap();
        assert_eq!(Token::Bool(false), tokens[0]);
    }
    #[test]
    fn test_number() {
        //integer
        let num = "1234567890";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(1234567890f64));

        let num = "+123";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(123f64));

        //float
        let num = "-0.001";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(-0.001));

        let num = ".001";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(0.001));

        // exponent
        let num = "1e-10";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(0.0000000001));

        let num = "+2E10";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(20000000000f64));
    }
}
