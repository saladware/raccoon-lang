use std::str::Chars;

#[derive(Debug)]
pub enum OperatorToken {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    Comma,
    ExclamationMark,
    Dot,
    Plus,
    Minus,
    Colon,
    Slash,
    Asterisk,
    Percent,
    DoubleDot,
    TripleDot,
    Equals,
    PlusEquals,
    MinusEquals,
    SlashEquals,
    AsteriskEquals,
    PercentEquals,
}

#[derive(Debug)]
pub enum BracketToken {
    OpenParenthesis,
    CloseParenthesis,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
}

#[derive(Debug)]
pub enum KeywordToken {
    For,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
}

#[derive(Debug)]
pub enum TokenType {
    StringLiteral(String),
    NumberLiteral(String),
    Operator(OperatorToken),
    Bracket(BracketToken),
    Keyword(KeywordToken),
    Identifier(String),
    Comment(String),
    EL,
}

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub column: usize,
    pub value: TokenType,
}

impl Token {
    pub fn new(value: TokenType, column: usize, line: usize) -> Self {
        Self { value, column, line }
    }
}

pub struct Lexer<'src> {
    code: Chars<'src>,
    tokens: Vec<Token>,
    line_counter: usize,
    column_counter: usize,
    wait_tokenization: Option<char>,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            code: source.chars(),
            tokens: Vec::new(),
            line_counter: 1,
            column_counter: 0,
            wait_tokenization: None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        return {
            if let Some(c) = self.wait_tokenization {
                self.wait_tokenization = None;
                Some(c)
            } else if let Some(value) = self.code.next() {
                if value == '\n' {
                    self.line_counter += 1;
                    self.column_counter = 0;
                } else {
                    self.column_counter += 1;
                }
                Some(value)
            } else { None }
        };
    }

    fn set_next_char(&mut self, c: char) {
        if let Some(value) = self.wait_tokenization {
            panic!("next char is already defined (char: '{}')", value);
        }
        self.wait_tokenization = Some(c);
    }

    fn tokenize_comment(&mut self, c: char) -> Result<TokenType, &'static str> {
        let mut token = String::from(c);
        loop {
            if let Some(c) = self.next_char() {
                if c == '\n' { break; } else { token.push(c) }
            } else { break; }
        }
        return Ok(TokenType::Comment(token));
    }
    fn tokenize_number(&mut self, value: char) -> Result<TokenType, &'static str> {
        let mut token = String::from(value);
        loop {
            // todo: 0xFF0000 0b11001 0o172
            if let Some(c) = self.next_char() {
                if c.is_ascii_digit() || c == '_' {
                    token.push(c)
                } else {
                    self.set_next_char(c);
                    break;
                }
            } else { break; }
        }
        return Ok(TokenType::NumberLiteral(token));
    }

    fn tokenize_string(&mut self, _value: char) -> Result<TokenType, &'static str> {
        // todo: \n \t \\
        let mut token = String::new();
        loop {
            if let Some(c) = self.next_char() {
                if c == '"' {
                    break;
                } else {
                    token.push(c)
                }
            } else {
                return Err("string does not have a closing quote");
            }
        }
        return Ok(TokenType::StringLiteral(token));
    }

    fn tokenize_name(&mut self, c: char) -> Result<TokenType, &'static str> {
        let mut token = String::from(c);
        loop {
            if let Some(c) = self.next_char() {
                if !c.is_whitespace() && !c.is_ascii_punctuation() || c == '_' {
                    token.push(c);
                } else {
                    self.set_next_char(c);
                    break;
                }
            } else {
                break;
            }
        }
        let result = match token.as_str() {
            "if" => TokenType::Keyword(KeywordToken::If),
            "else" => TokenType::Keyword(KeywordToken::Else),
            "while" => TokenType::Keyword(KeywordToken::While),
            "for" => TokenType::Keyword(KeywordToken::For),
            "false" => TokenType::Keyword(KeywordToken::False),
            "true" => TokenType::Keyword(KeywordToken::True),
            "null" => TokenType::Keyword(KeywordToken::Null),
            "return" => TokenType::Keyword(KeywordToken::Return),
            _ => TokenType::Identifier(token)
        };
        return Ok(result);
    }

    fn tokenize_op_with_eq(&mut self, op: OperatorToken, op_eq: OperatorToken) -> TokenType {
        let res = if let Some(value) = self.next_char() {
            if value == '=' {
                op_eq
            } else {
                self.set_next_char(value);
                op
            }
        } else {
            op
        };
        TokenType::Operator(res)
    }

    fn tokenize_operation(&mut self, c: char) -> Result<TokenType, &'static str> {
        return match c {
            '(' => Ok(TokenType::Bracket(BracketToken::OpenParenthesis)),
            ')' => Ok(TokenType::Bracket(BracketToken::CloseParenthesis)),
            '[' => Ok(TokenType::Bracket(BracketToken::OpenSquare)),
            ']' => Ok(TokenType::Bracket(BracketToken::CloseSquare)),
            '{' => Ok(TokenType::Bracket(BracketToken::OpenCurly)),
            '}' => Ok(TokenType::Bracket(BracketToken::CloseCurly)),
            '+' => Ok(self.tokenize_op_with_eq(OperatorToken::Plus, OperatorToken::PlusEquals)),
            '-' => Ok(self.tokenize_op_with_eq(OperatorToken::Minus, OperatorToken::MinusEquals)),
            '*' => Ok(self.tokenize_op_with_eq(OperatorToken::Asterisk, OperatorToken::AsteriskEquals)),
            '/' => Ok(self.tokenize_op_with_eq(OperatorToken::Slash, OperatorToken::SlashEquals)),
            '%' => Ok(self.tokenize_op_with_eq(OperatorToken::Percent, OperatorToken::PercentEquals)),
            '>' => Ok(self.tokenize_op_with_eq(OperatorToken::Gt, OperatorToken::Ge)),
            '<' => Ok(self.tokenize_op_with_eq(OperatorToken::Lt, OperatorToken::Le)),
            '!' => Ok(self.tokenize_op_with_eq(OperatorToken::ExclamationMark, OperatorToken::Ne)),
            '=' => Ok(self.tokenize_op_with_eq(OperatorToken::Equals, OperatorToken::Eq)),
            ',' => Ok(TokenType::Operator(OperatorToken::Comma)),
            ':' => Ok(TokenType::Operator(OperatorToken::Colon)),
            '.' => {
                if let Some(value) = self.next_char() {
                    if value == '.' {
                        if let Some(value2) = self.next_char() {
                            if value2 == '.' {
                                Ok(TokenType::Operator(OperatorToken::TripleDot))
                            }
                            else {
                                self.set_next_char(value2);
                                Ok(TokenType::Operator(OperatorToken::DoubleDot))
                            }
                        }
                        else {
                            Ok(TokenType::Operator(OperatorToken::DoubleDot))
                        }
                    }
                    else {
                        self.set_next_char(value);
                        Ok(TokenType::Operator(OperatorToken::Dot))
                    }
                }
                else {
                    Ok(TokenType::Operator(OperatorToken::Dot))
                }
            }
            _ => Err("unknown operation")
        };
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        loop {
            if let Some(symbol) = self.next_char() {
                let l = self.line_counter;
                let c = self.column_counter;
                let token = match symbol {
                    '#' => self.tokenize_comment(symbol),
                    '"' => self.tokenize_string(symbol),
                    '\n' | ';' => {
                        if let Some(Token { value: TokenType::EL, .. }) = self.tokens.last() {
                            continue;
                        }
                        Ok(TokenType::EL)
                    }
                    d if d.is_ascii_digit() => self.tokenize_number(symbol),
                    space if space.is_ascii_whitespace() => continue,
                    x if x.is_ascii_punctuation() => self.tokenize_operation(symbol),
                    _ => self.tokenize_name(symbol)
                };
                match token {
                    Ok(value) => {
                        self.tokens.push(Token::new(value, c, l))
                    }
                    Err(msg) => {
                        eprintln!("Lexer Error: {} (line {}, column {})", msg, l, c);
                        std::process::exit(1);
                    }
                }
            } else {
                match self.tokens.last() {
                    Some(Token { value: TokenType::EL, .. }) => {}
                    _ => {
                        self.tokens.push(
                            Token::new(TokenType::EL, self.column_counter, self.line_counter)
                        );
                    }
                }
                return self.tokens;
            }
        }
    }
}