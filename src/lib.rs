mod parser;
mod tokenizer;

pub use parser::Operation;
type float = f64;

pub fn evaluate(input: &str) -> Result<float, EvalError> {
    let tokens = tokenizer::tokenize(input)?;
    let mut parser = parser::Parser::new(tokens);
    parser.parse()?;

    return Ok(parser.root.evaluate());
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    InvalidCharacter(char),
    InvalidNumber(String),
    UnexpectedEnd,
    UnexpectedToken(String),
    DivisionByZero,
    OutOfBoundsCharacter,
    Unknown,
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::{evaluate, float};

    fn calculate(input: &str) -> float {
        return evaluate(input).unwrap();
    }

    #[test]
    fn test_addition() {
        assert_eq!(calculate("3 + 2"), 5.0);
        assert_eq!(calculate("3 + 2 + 1"), 6.0);
        assert_eq!(calculate("9 + 9 + 0"), 18.0);
        assert_eq!(calculate("1 + 7 + 5 + 3 + 8"), 24.0);
        assert_eq!(calculate("3"), 3.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate("3 - 2"), 1.0);
        assert_eq!(calculate("3 - 2 - 1"), 0.0);
        assert_eq!(calculate("15 - 2"), 13.0);
        assert_eq!(calculate("15 - 1 - 1 - 1 - 1"), 11.0);
        assert_eq!(calculate("2 + 6 - 1 + 7 - 2"), 12.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate("3 * 2"), 6.0);
        assert_eq!(calculate("3 * 5"), 15.0);
        assert_eq!(calculate("1 * 2"), 2.0);
        assert_eq!(calculate("3 * 2 * 6"), 36.0);
        assert_eq!(calculate("3 + 2 * 2"), 7.0);
        assert_eq!(calculate("7 * 3 + 5"), 26.0);
        assert_eq!(calculate("3 + 5 * 2 + 7 * 5"), 48.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate("6 / 2"), 3.0);
        assert_eq!(calculate("6 / 2"), 3.0);
        assert_eq!(calculate("16 / 2 / 2"), 4.0);
        assert_eq!(calculate("27 / 3"), 9.0);
        assert_eq!(calculate("5 / 5"), 1.0);
        assert_eq!(calculate("1 + 7 / 7"), 2.0);
        assert_eq!(calculate("8 / 4 + 2 / 1"), 4.0);
    }

    #[test]
    fn test_spaces() {
        assert_eq!(calculate("1+1"), 2.0);
        assert_eq!(calculate("1 + 1"), 2.0);
        assert_eq!(calculate("1 +1"), 2.0);
        assert_eq!(calculate("1+ 1"), 2.0);
        assert_eq!(calculate("5 * 6/3"), 10.0);
        assert_eq!(calculate("1+3*2"), 7.0);
    }
}
