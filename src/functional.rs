use crate::EvalError;
use crate::Float;
use crate::parser::Operation;
use std::collections::HashMap;

// keychains
pub struct Keychain {
    pub(crate) hashmap: HashMap<String, Operation>,
}
impl Keychain {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }
    // hopefully temporary
    pub fn define_variable(&mut self, name: String, value: Operation) {
        self.hashmap.insert(name, value);
    }
}
pub type Function = Operation;
impl Function {
    pub fn get_substituted(&self, keychain: &Keychain) -> Result<Operation, EvalError> {
        match self {
            Operation::Number(n) => Ok(Operation::Number(*n)),
            Operation::Addition(left, right) => Ok(Operation::Addition(
                Box::new(left.get_substituted(keychain)?),
                Box::new(right.get_substituted(keychain)?),
            )),
            Operation::Subtraction(left, right) => Ok(Operation::Subtraction(
                Box::new(left.get_substituted(keychain)?),
                Box::new(right.get_substituted(keychain)?),
            )),
            Operation::Multiplication(left, right) => Ok(Operation::Multiplication(
                Box::new(left.get_substituted(keychain)?),
                Box::new(right.get_substituted(keychain)?),
            )),
            Operation::Division(left, right) => Ok(Operation::Division(
                Box::new(left.get_substituted(keychain)?),
                Box::new(right.get_substituted(keychain)?),
            )),

            Operation::Variable(n) => Ok(Operation::Number(Self::resolve_variable(keychain, n)?)),
        }
    }
    pub fn eval_for(&self, keychain: &Keychain) -> Result<Float, EvalError> {
        let sub = self.get_substituted(keychain)?;
        return sub.eval();
    }

    fn resolve_variable(keychain: &Keychain, varname: &str) -> Result<Float, EvalError> {
        if keychain.hashmap.contains_key(varname) {
            return Ok(keychain
                .hashmap
                .get(varname)
                .ok_or_else(|| EvalError::Unknown(0))?
                .eval_for(keychain)?);
        }
        return Err(EvalError::UndefinedVariable(0, varname.to_string()));
    }
}

#[macro_export]
macro_rules! new_keychain {
    ($($key:ident = $value:expr),* $(,)?) => {{
        let mut keychain = $crate::Keychain::new();

        $(
            keychain.define_variable(stringify!($key).to_string(), $crate::Operation::Number($value));
        )*

        keychain
    }};
}

#[macro_export]
macro_rules! eval_immediate_func {
    ($expr:expr, $($key:ident = $value:expr),* $(,)?) => {{
        (|| -> Result<$crate::Float, $crate::EvalError> {
            let func = $crate::Function::try_new($expr)?;
            let mut keychain = $crate::Keychain::new();

            $(
                keychain.define_variable(
                    stringify!($key).to_string(),
                    $crate::Operation::Number($value),
                );
            )*

            func.eval_for(&keychain)
        })()
    }};
}

// modifying the equation
impl Function {
    pub fn get_double(&self) -> Function {
        Operation::Multiplication(Box::new(Operation::Number(2.0)), Box::new(self.clone()))
    }
    pub fn get_half(&self) -> Function {
        Operation::Multiplication(Box::new(Operation::Number(0.5)), Box::new(self.clone()))
    }
    pub fn get_product(&self, num: Float) -> Function {
        Operation::Multiplication(Box::new(Operation::Number(num)), Box::new(self.clone()))
    }
    pub fn get_sum(&self, num: Float) -> Function {
        Operation::Addition(Box::new(Operation::Number(num)), Box::new(self.clone()))
    }
}
