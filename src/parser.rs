use crate::{EvalError, Float, tokenizer::TokenType};

#[derive(Clone, Debug)]
pub enum Operation {
    Number(Float),
    Variable(String),
    Addition(Box<Operation>, Box<Operation>),
    Subtraction(Box<Operation>, Box<Operation>),
    Multiplication(Box<Operation>, Box<Operation>),
    Division(Box<Operation>, Box<Operation>),
}

impl Operation {
    pub fn evaluate(&self) -> Result<Float, EvalError> {
        match self {
            Operation::Number(n) => Ok(*n),
            Operation::Addition(left, right) => Ok(left.evaluate()? + right.evaluate()?),
            Operation::Subtraction(left, right) => Ok(left.evaluate()? - right.evaluate()?),
            Operation::Multiplication(left, right) => Ok(left.evaluate()? * right.evaluate()?),
            Operation::Division(left, right) => Ok(left.evaluate()? / right.evaluate()?),

            Operation::Variable(_n) => Err(EvalError::InvalidEvaluationWithVariable(0)),
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
        // parantheses check
        let left_p = self
            .tokens
            .iter()
            .filter(|&t| t.tk_type == TokenType::LParen)
            .count();
        let right_p = self
            .tokens
            .iter()
            .filter(|&t| t.tk_type == TokenType::RParen)
            .count();
        if left_p != right_p {
            return Err(EvalError::UnmatchedParantheses(self.i));
        }

        let expr = self.parse_addition()?;
        if self.i != self.tokens.len() {
            return Err(EvalError::UnexpectedToken(
                self.i,
                self.tokens
                    .get(self.i)
                    .ok_or(EvalError::OutOfBoundsCharacter(self.i))?
                    .lexeme
                    .clone(),
            ));
        }
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
                    left = Operation::Addition(Box::from(left.clone()), Box::from(right));
                }
                TokenType::Minus => {
                    self.i += 1;
                    let right = self.parse_term()?;
                    left = Operation::Subtraction(Box::new(left.clone()), Box::new(right));
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
                    if right.evaluate()? == 0.0 {
                        return Err(EvalError::DivisionByZero(self.i));
                    }

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
        let tkn = self
            .tokens
            .get(self.i)
            .ok_or(EvalError::UnexpectedEnd(self.i))?;
        match tkn.tk_type {
            TokenType::Number => {
                let num: f64 = tkn
                    .lexeme
                    .parse()
                    .map_err(|_| EvalError::InvalidNumber(self.i, tkn.lexeme.clone()))?;
                return Ok(Operation::Number(num));
            }
            TokenType::LParen => {
                self.i += 1;
                let inner = self.parse_addition()?;
                if self
                    .tokens
                    .get(self.i)
                    .ok_or(EvalError::UnexpectedEnd(self.i))?
                    .tk_type
                    != TokenType::RParen
                {
                    return Err(EvalError::UnmatchedParantheses(self.i));
                }
                return Ok(inner);
            }
            TokenType::RParen => {
                return Err(EvalError::EmptyParantheses(self.i));
            }
            _ => return Err(EvalError::InvalidFactor(self.i)),
        }
    }
}
