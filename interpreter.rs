
use parser::{Parser, AST};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Interpreter {
    parser: Parser,
    symbols: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new(source: String) -> Interpreter {
        Interpreter {
            parser: Parser::new(source),
            symbols: HashMap::new(),
        }
    }

    pub fn new_continue(source: String, previous: HashMap<String, i64>) -> Interpreter {
        Interpreter {
            parser: Parser::new(source),
            symbols: previous,
        }
    }

    pub fn interpret(&mut self) -> HashMap<String, i64> {
        if let AST::Program(assignments) = self.parser.parse().unwrap() {
            for assignment in assignments {
                if let AST::Assignment(ref id, ref expr) = *assignment {
                    let symbol: String;
                    if let AST::Identifier(ref t2) = **id {
                        symbol = t2.clone();
                    } else {
                        panic!("Expected an identifier!");
                    }
                    let value = self.eval(&expr);
                    self.symbols.insert(symbol, value);
                } else {
                    panic!("Expected an assignment statement.");
                }
            }
        }
        self.symbols.clone()
    }

    pub fn print_symbols(&mut self) {
        println!("{:?}", self.symbols);
    }

    fn eval(&self, node: &Box<AST>) -> i64 {
        match **node {
            AST::Addition      (ref lhs, ref rhs) => self.eval(&lhs) + self.eval(&rhs),
            AST::Subtraction   (ref lhs, ref rhs) => self.eval(&lhs) - self.eval(&rhs),
            AST::Multiplication(ref lhs, ref rhs) => self.eval(&lhs) * self.eval(&rhs),
            AST::Division      (ref lhs, ref rhs) => self.eval(&lhs) / self.eval(&rhs),
            AST::Identifier    (ref id)           => {
                match self.symbols.get(id) {
                    Some(int_val) => *int_val,
                    None => panic!("Could not find {:?} in the symbol map!", id),
                }
            }
            AST::Integer(int_val) => int_val,
            _ => panic!("Unexpected expression node!")
        }
    }
}
