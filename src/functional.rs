use crate::EvalError;
use crate::Float;
use crate::parser::Operation;
use std::collections::HashMap;

pub struct Keychain {
    pub(crate) hashmap: HashMap<char, Operation>,
}
impl Keychain {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }
    // hopefully temporary
    pub fn define_variable(&mut self, name: char, value: Operation) {
        self.hashmap.insert(name, value);
    }
}
pub type Function = Operation;
impl Function {
    pub fn evaluate_with_keychain(&self, keychain: &Keychain) -> Result<Float, EvalError> {
        match self {
            Operation::Number(n) => Ok(*n),
            Operation::Addition(left, right) => {
                Ok(left.evaluate_with_keychain(keychain)?
                    + right.evaluate_with_keychain(keychain)?)
            }
            Operation::Subtraction(left, right) => {
                Ok(left.evaluate_with_keychain(keychain)?
                    - right.evaluate_with_keychain(keychain)?)
            }
            Operation::Multiplication(left, right) => {
                Ok(left.evaluate_with_keychain(keychain)?
                    * right.evaluate_with_keychain(keychain)?)
            }
            Operation::Division(left, right) => {
                Ok(left.evaluate_with_keychain(keychain)?
                    / right.evaluate_with_keychain(keychain)?)
            }

            Operation::Variable(n) => Self::resolve_variable(
                keychain,
                n.chars()
                    .next()
                    .ok_or_else(|| EvalError::InvalidVariableName(0, n.clone()))?,
            ),
        }
    }

    fn resolve_variable(keychain: &Keychain, varname: char) -> Result<Float, EvalError> {
        if keychain.hashmap.contains_key(&varname) {
            return Ok(keychain
                .hashmap
                .get(&varname)
                .ok_or_else(|| EvalError::Unknown(0))?
                .evaluate_with_keychain(keychain)?);
        }
        return Err(EvalError::UndefinedVariable(0, varname));
    }
}
