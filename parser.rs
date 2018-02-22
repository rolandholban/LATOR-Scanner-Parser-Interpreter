// prog    -> { asgn }*
// asgn    -> Identifier Equal expr SemiColon
// expr    -> term expr'
// expr'   -> (Plus | Minus) term expr' | epsilon
// term    -> factor term'
// term'   -> (Star | Slash) factor term' | epsilon
// factor  -> IntegerLiteral | LeftParen expr RightParen | Identifier

use scanner::{Scanner, Token};

#[derive(Debug)]
pub enum AST {
    Program(Vec<Box<AST>>),
    Assignment(Box<AST>, Box<AST>),
    Addition(Box<AST>, Box<AST>),
    Subtraction(Box<AST>, Box<AST>),
    Multiplication(Box<AST>, Box<AST>),
    Division(Box<AST>, Box<AST>),
    Identifier(String),
    Integer(i64),
}

#[derive(Debug)]
pub struct Parser {
    scanner: Scanner,
    token: Option<Token>,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            scanner: Scanner::new(source),
            token: None
        }
    }

    pub fn parse(&mut self) -> Option<AST> {
        // Initialize self.token
        self.advance();

        // Start parsing
        let ast = self.prog();

        // Return None if the input is not empty
        match self.scanner.empty() {
            true => Some(ast),
            false => None,
        }
    }

    fn advance(&mut self) {
        self.token = self.scanner.get_next_token();
    }

    // prog    -> { asgn }*
    fn prog(&mut self) -> AST {
        let mut assignments: Vec<Box<AST>> = Vec::new();
        while let Some(Token::Identifier(_)) = self.token {
            assignments.push(Box::new(self.asgn()));
        }
        AST::Program(assignments)
    }

    // asgn    -> Identifier Equal expr SemiColon
    fn asgn(&mut self) -> AST {
        match self.token {
            Some(Token::Identifier(_)) => {
                let mut new_id = String::new();
                if let Some(Token::Identifier(ref id)) = self.token {
                    new_id = id.clone();
                }
                self.advance();
                match self.token {
                    Some(Token::Equal) => {
                        self.advance();
                        let ast = self.expr();
                        match self.token {
                            Some(Token::SemiColon) => {
                                self.advance();
                                AST::Assignment(
                                    Box::new(AST::Identifier(new_id)),
                                    Box::new(ast)
                                )
                            }
                            _ => panic!("Expected a SemiColon!")
                        }
                    }
                    _ => panic!("Expected an Equal!")
                }
            }
            _ => panic!("Expected an Identifier!")
        }
    }

    // expr    -> term expr'
    fn expr(&mut self) -> AST {
        let lhs = self.term();
        self.expr_p(lhs)
    }

    // expr'   -> (Plus | Minus) term expr' | epsilon
    fn expr_p(&mut self, lhs: AST) -> AST {
        match self.token {
            Some(Token::Plus) => {
                self.advance();
                let rhs = self.term();
                let ast = AST::Addition(Box::new(lhs), Box::new(rhs));
                self.expr_p(ast)
            }
            Some(Token::Minus) => {
                self.advance();
                let rhs = self.term();
                let ast = AST::Subtraction(Box::new(lhs), Box::new(rhs));
                self.expr_p(ast)
            }
            _ => lhs,
        }
    }

    // term    -> factor term'
    fn term(&mut self) -> AST {
        let lhs = self.factor();
        self.term_p(lhs)
    }

    // term'   -> (Star | Slash) factor term' | epsilon
    fn term_p(&mut self, lhs: AST) -> AST {
        match self.token {
            Some(Token::Star) => {
                self.advance();
                let rhs = self.factor();
                let ast = AST::Multiplication(Box::new(lhs), Box::new(rhs));
                self.term_p(ast)
            }
            Some(Token::Slash) => {
                self.advance();
                let rhs = self.factor();
                let ast = AST::Division(Box::new(lhs), Box::new(rhs));
                self.term_p(ast)
            }
            _ => lhs,
        }
    }

    // factor  -> IntegerLiteral | LeftParen expr RightParen
    fn factor(&mut self) -> AST {
        match self.token {
            Some(Token::IntegerLiteral(val)) => {
                self.advance();
                AST::Integer(val)
            }
            Some(Token::LeftParen) => {
                self.advance();
                let ast = self.expr();
                match self.token {
                    Some(Token::RightParen) => self.advance(),
                    _ => panic!("Expected: {:?}, Found: {:?}", Token::RightParen, self.token),
                }
                ast
            }
            Some(Token::Identifier(_)) => {
                let mut new_id = String::new();
                if let Some(Token::Identifier(ref id)) = self.token {
                    new_id = id.clone();
                }
                self.advance();
                AST::Identifier(new_id)
            }
            _ => panic!("Unexpected token: {:?}", self.token),
        }
    }
}
