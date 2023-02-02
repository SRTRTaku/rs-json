#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    WhiteSpace,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
}

/// parse json string and divide it to tokens.
pub struct Lexer<`a> {
    /// point head character
    chars : std::iter::Peekable<std::str::Chars<'a>>
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
    pub fn new(input: &str) -> Lexe {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    /// divide string to tokens
    pub tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
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
        unimplemented!()
    }

    /// parse null string
    fn parse_null_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// parse (true|false) string
    fn parse_bool_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }
    /// parse number string
    fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
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

