use crate::{EvalError, Float, tokenizer::TokenType};

#[derive(Clone, Debug)]
pub enum Operation {
    Number(Float),
    Addition(Box<Operation>, Box<Operation>),
    Subtraction(Box<Operation>, Box<Operation>),
    Multiplication(Box<Operation>, Box<Operation>),
    Division(Box<Operation>, Box<Operation>),
}

impl Operation {
    pub fn evaluate(&self) -> Float {
        match self {
            Operation::Number(n) => *n,
            Operation::Addition(left, right) => left.evaluate() + right.evaluate(),
            Operation::Subtraction(left, right) => left.evaluate() - right.evaluate(),
            Operation::Multiplication(left, right) => left.evaluate() * right.evaluate(),
            Operation::Division(left, right) => left.evaluate() / right.evaluate(),
        }
    }
}

pub struct Parser {
    i: usize,
    tokens: Vec<super::tokenizer::Token>,
    pub root: Operation,
}

impl Parser {
    pub fn new(tkns: Vec<super::tokenizer::Token>) -> Parser {
        return Parser {
            i: 0,
            tokens: tkns,
            root: Operation::Number(0.0),
        };
    }
    pub fn parse(&mut self) -> Result<(), EvalError> {
        let expr = self.parse_addition()?;
        self.root = expr;
        return Ok(());
    }
    fn parse_addition(&mut self) -> Result<Operation, EvalError> {
        let mut left: Operation = self.parse_term()?;

        while let Some(tkn) = self.tokens.get(self.i) {
            match tkn.tk_type {
                TokenType::Plus => {
                    self.i += 1;
                    let right = self.parse_term()?;
                    left = Operation::Addition(Box::new(left.clone()), Box::new(right.clone()));
                }
                TokenType::Minus => {
                    self.i += 1;
                    let right = self.parse_term()?;
                    left = Operation::Subtraction(Box::new(left.clone()), Box::new(right.clone()));
                }
                _ => {
                    break;
                }
            }

            if self.i >= self.tokens.len() {
                break;
            }
        }

        return Ok(left);
    }
    fn parse_term(&mut self) -> Result<Operation, EvalError> {
        let mut left = self.parse_factor()?;

        self.i += 1;
        while let Some(tkn) = self.tokens.get(self.i) {
            match tkn.tk_type {
                TokenType::Times => {
                    self.i += 1;
                    let right = self.parse_factor()?;

                    self.i += 1;

                    left = Operation::Multiplication(Box::new(left.clone()), Box::new(right));
                }
                TokenType::Over => {
                    self.i += 1;
                    let right = self.parse_factor()?;

                    self.i += 1;

                    left = Operation::Division(Box::new(left.clone()), Box::new(right));
                }

                _ => {
                    break;
                }
            }

            if self.i >= self.tokens.len() {
                break;
            }
        }

        return Ok(left);
    }
    fn parse_factor(&mut self) -> Result<Operation, EvalError> {
        let tkn = self.tokens.get(self.i).ok_or(EvalError::UnexpectedEnd)?;
        match tkn.tk_type {
            TokenType::Number => {
                return Ok(Operation::Number(tkn.lexeme.parse().unwrap()));
            }
            TokenType::LParen => {
                self.i += 1;
                let inner = self.parse_addition().unwrap();
                if self
                    .tokens
                    .get(self.i)
                    .ok_or(EvalError::UnexpectedEnd)?
                    .tk_type
                    != TokenType::RParen
                {
                    return Err(EvalError::UnmatchedParantheses);
                }
                return Ok(inner);
            }
            _ => return Err(EvalError::InvalidFactor),
        }
    }
}
