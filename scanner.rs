
#[derive(Debug)]
pub enum Token {
    // Punctuators
    Plus, Minus, Star, Slash, Equal,
    SemiColon, LeftParen, RightParen,

    // Tokens with data
    IntegerLiteral(i64),
    Identifier(String),
}

fn is_ascii_alpha(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct Scanner {
    chars: Vec<char>,
    cursor: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            chars: source.chars().collect(),
            cursor: 0,
        }
    }

    pub fn empty(&mut self) -> bool {
        self.skip_whitespace();
        self.done()
    }

    pub fn get_next_token(&mut self) -> Option<Token> {

        self.skip_whitespace();

        if self.done() {
            return None;
        }

        let token = match self.chars[self.cursor] {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '=' => Some(Token::Equal),
            ';' => Some(Token::SemiColon),
            '0' ... '9' => Some(self.integer_literal()),
            c if is_ascii_alpha(c) => Some(self.identifier()),
            _ => None
        };
        self.cursor += 1;
        token
    }
    fn done(&self) -> bool {
        self.cursor >= self.chars.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.done() && self.chars[self.cursor].is_whitespace() {
            self.cursor += 1;
        }
    }

    fn integer_literal(&mut self) -> Token {
        let mut i = String::new();
        while !self.done() && self.chars[self.cursor].is_digit(10) {
            i.push(self.chars[self.cursor]);
            self.cursor += 1;
        }
        self.cursor -= 1;
        Token::IntegerLiteral(i.parse::<i64>().unwrap())

    }

    fn identifier(&mut self) -> Token {
        let mut id = String::new();
        while !self.done() &&
            (is_ascii_alpha(self.chars[self.cursor]) ||
            self.chars[self.cursor].is_digit(10) ||
            self.chars[self.cursor] == '_')
        {
            id.push(self.chars[self.cursor]);
            self.cursor += 1;
        }
        self.cursor -= 1;
        Token::Identifier(id)
    }
}
