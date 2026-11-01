mod functional;
mod parser;
mod tokenizer;

pub use functional::Function;
pub use functional::Keychain;
pub use parser::Operation;
pub type Float = f64;

pub fn eval(input: &str) -> Result<Float, EvalError> {
    let expr = generate_function(input)?;
    let result = expr.eval()?;

    return Ok(result);
}
pub fn eval_for(input: &str, keychain: &Keychain) -> Result<Float, EvalError> {
    let expr = generate_function(input)?;
    let result = expr.eval_for(keychain)?;

    return Ok(result);
}
pub fn generate_function(input: &str) -> Result<Operation, EvalError> {
    let tokens = tokenizer::tokenize(input)?;
    let mut parser = parser::Parser::new(tokens);
    parser.parse()?;

    return Ok(parser.root);
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    InvalidCharacter(usize, char),
    InvalidNumber(usize, String),
    UnexpectedEnd(usize),
    UnexpectedToken(usize, String),
    DivisionByZero(usize),
    OutOfBoundsCharacter(usize),
    InvalidFactor(usize),
    UnmatchedParantheses(usize),
    EmptyParantheses(usize),
    InvalidEvaluationWithVariable(usize),
    UndefinedVariable(usize, String),
    InvalidVariableName(usize, String),
    Unknown(usize),
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::{
        Float, Function, Keychain, eval, eval_immediate_func, generate_function, new_keychain,
    };

    fn calculate(input: &str) -> Float {
        return eval(input).unwrap();
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

    #[test]
    fn test_parantheses() {
        assert_eq!(calculate("2 * (3 + 2)"), 10.0);
        assert_eq!(calculate("(5 + 1) * 3"), 18.0);
        assert_eq!(calculate("(1+2)*5"), 15.0);
        assert_eq!(calculate("( 5 + 2)*  7"), 49.0);
    }

    #[test]
    fn test_keychains() {
        let mut keychain = Keychain::new();
        keychain.define_variable("a".to_string(), crate::Operation::Number(6.0));
        keychain.define_variable(
            "b".to_string(),
            crate::Operation::Variable(String::from("a")),
        );

        let expr = generate_function("7 * a + 3").unwrap();
        assert_eq!(expr.eval_for(&keychain).unwrap(), 45.0);
        let expr2 = generate_function("5 + 2 * b").unwrap();
        assert_eq!(expr2.eval_for(&keychain).unwrap(), 17.0);
    }

    #[test]
    fn test_keychain_generation() {
        let keychain1 = new_keychain!(a = 5.0, b = 3.0);
        let keychain2 = new_keychain!(a = 7.0, b = 10.0);
        let expr: Function = Function::try_new("a + b").unwrap();

        assert_eq!(expr.eval_for(&keychain1).unwrap(), 8.0);
        assert_eq!(expr.eval_for(&keychain2).unwrap(), 17.0);
    }

    #[test]
    fn test_variable_equations() {
        let keychain = new_keychain!(a = 2.0, b = 3.0, c = 5.0);
        let e1: Function = Function::try_new("a / a").unwrap();
        let e2: Function = Function::try_new("c / (a + b)").unwrap();

        assert_eq!(e1.eval_for(&keychain).unwrap(), 1.0);
        assert_eq!(e2.eval_for(&keychain).unwrap(), 1.0)
    }

    #[test]
    fn test_string_variable_names() {
        let keychain = new_keychain!(
            pi = 3.14,
            e = 2.71,
            g_earth = 9.81,
            density_sea_level = 1.225
        );
        assert_eq!(
            generate_function("pi").unwrap().eval_for(&keychain),
            Ok(3.14)
        );
        assert_eq!(
            generate_function("e").unwrap().eval_for(&keychain),
            Ok(2.71)
        );
        assert_eq!(
            generate_function("g_earth").unwrap().eval_for(&keychain),
            Ok(9.81)
        );
        assert_eq!(
            generate_function("density_sea_level")
                .unwrap()
                .eval_for(&keychain),
            Ok(1.225)
        );
        assert_eq!(
            generate_function("pi + e").unwrap().eval_for(&keychain),
            Ok(5.85)
        );
        assert_eq!(
            generate_function("pi + density_sea_level + e")
                .unwrap()
                .eval_for(&keychain),
            Ok(7.075)
        );
    }

    #[test]
    fn test_evalulate_function_macro() {
        assert_eq!(eval_immediate_func!("3 * x + 5", x = 2.0).unwrap(), 11.0);
        assert_eq!(
            eval_immediate_func!("e + pi", e = 2.71, pi = 3.14).unwrap(),
            5.85
        );
        assert_eq!(
            eval_immediate_func!("a + b + c", a = 2.0, b = 3.0, c = 1.0).unwrap(),
            6.0
        );
    }

    #[test]
    fn test_function_getter_modifiers() {
        let func1: Function = Function::try_new("3 * x + 1").unwrap();
        let keychain: Keychain = new_keychain!(x = 4.0);

        assert_eq!(func1.eval_for(&keychain).unwrap(), 13.0);
        assert_eq!(func1.get_double().eval_for(&keychain).unwrap(), 26.0);
        assert_eq!(
            func1.get_double().get_double().eval_for(&keychain).unwrap(),
            52.0
        );
        assert_eq!(
            func1.get_double().get_half().eval_for(&keychain).unwrap(),
            func1.eval_for(&keychain).unwrap()
        );

        assert_eq!(func1.get_product(3.0).eval_for(&keychain).unwrap(), 39.0);
        assert_eq!(
            func1
                .get_product(3.0)
                .get_sum(2.0)
                .eval_for(&keychain)
                .unwrap(),
            41.0
        );
        assert_eq!(
            func1
                .get_sum(5.0)
                .get_product(3.0)
                .eval_for(&keychain)
                .unwrap(),
            54.0
        );
    }

    #[test]
    fn test_function_modifiers() {
        let mut func1: Function = Function::try_new("2 * x + 1").unwrap();
        let keychain: Keychain = new_keychain!(x = 3.0);

        assert_eq!(func1.eval_for(&keychain).unwrap(), 7.0);
        func1.multiply(4.0);
        assert_eq!(func1.eval_for(&keychain).unwrap(), 28.0);
        func1.divide(2.0);
        assert_eq!(func1.eval_for(&keychain).unwrap(), 14.0);
        func1.add(3.0);
        assert_eq!(func1.eval_for(&keychain).unwrap(), 17.0);
        func1.subtract(6.0);
        assert_eq!(func1.eval_for(&keychain).unwrap(), 11.0);
    }
}
