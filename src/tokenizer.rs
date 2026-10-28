use crate::EvalError;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Times,
    Over,
}

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub tk_type: TokenType,
}

pub fn tokenize(plain: &str) -> Result<Vec<Token>, EvalError> {
    let mut i = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while i < plain.chars().count() {
        let c = plain
            .chars()
            .nth(i)
            .ok_or(EvalError::OutOfBoundsCharacter)
            .unwrap();
        if c.is_ascii_digit() {
            let mut num_string = String::new();
            while i < plain.chars().count()
                && (plain
                    .chars()
                    .nth(i)
                    .expect("out of bounds")
                    .is_ascii_digit()
                    || plain
                        .chars()
                        .nth(i)
                        .ok_or(EvalError::OutOfBoundsCharacter)
                        .unwrap()
                        == '.')
            {
                num_string.push(
                    plain
                        .chars()
                        .nth(i)
                        .ok_or(EvalError::OutOfBoundsCharacter)
                        .unwrap(),
                );
                i += 1;
            }

            tokens.push(Token {
                lexeme: num_string,
                tk_type: TokenType::Number,
            });
            i -= 1;
        } else if c == '+' {
            tokens.push(Token {
                lexeme: String::from(c),
                tk_type: TokenType::Plus,
            });
        } else if c == '-' {
            tokens.push(Token {
                lexeme: String::from(c),
                tk_type: TokenType::Minus,
            });
        } else if c == '*' {
            tokens.push(Token {
                lexeme: String::from(c),
                tk_type: TokenType::Times,
            });
        } else if c == '/' {
            tokens.push(Token {
                lexeme: String::from(c),
                tk_type: TokenType::Over,
            });
        } else if c != ' ' {
            return Err(EvalError::InvalidCharacter(c));
        }

        i += 1;
    }

    return Ok(tokens);
}
