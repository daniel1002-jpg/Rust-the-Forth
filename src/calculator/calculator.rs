use super::calculator_errors::CalculatorError;
use crate::errors::Error;

/// A simple calculator that can perform basic arithmetic operations
/// such as addition, subtraction, multiplication, and division.
/// 
pub struct Calculator {}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    /// Creates a new instance of the Calculator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_forth::calculator::Calculator;
    /// let calculator = Calculator::new();
    /// ```
    pub fn new() -> Self {
        Calculator {}
    }

    fn add(&self, n1: i16, n2: i16) -> i16 {
        n1 + n2
    }

    fn subtract(&self, n1: i16, n2: i16) -> i16 {
        n1 - n2
    }

    fn multiply(&self, n1: i16, n2: i16) -> i16 {
        n1 * n2
    }

    fn divide(&self, n1: i16, n2: i16) -> Result<i16, Error> {
        match n2 {
            0 => Err(CalculatorError::DivisionByZero.into()),
            _ => Ok(n1 / n2),
        }
    }

    /// Performs the specified arithmetic operation on two numbers.
    ///
    /// # Arguments
    ///
    /// * `n1` - The first number.
    /// * `n2` - The second number.
    /// * `operation` - The operation to perform. It can be one of the following:
    ///   - "+" for addition
    ///   - "-" for subtraction
    ///   - "*" for multiplication
    ///   - "/" for division
    ///
    /// Returns the result of the operation as an `i16` value.   
    pub fn calculate(&self, n1: i16, n2: i16, operation: &str) -> Result<i16, Error> {
        match operation {
            "+" => Ok(self.add(n1, n2)),
            "-" => Ok(self.subtract(n1, n2)),
            "*" => Ok(self.multiply(n1, n2)),
            "/" => self.divide(n1, n2),
            _ => Err(CalculatorError::UndifiedOperation.into()),
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::calculator::{
        calculator::Calculator,
        calculator_errors::CalculatorError,
    };

    #[test]
    fn a_calculator_can_add_correctly() {
        let calculator = Calculator::new();
        let n1 = 2;
        let n2 = 4;
        let expected_result = 6;

        let result = calculator.add(n1, n2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn a_calculator_can_subtract_correctly() {
        let calculator = Calculator::new();
        let n1 = 4;
        let n2 = 2;
        let expected_result = 2;

        let result = calculator.subtract(n1, n2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn a_calculator_can_multiply_correctly() {
        let calculator = Calculator::new();
        let n1 = 4;
        let n2 = 2;
        let expected_result = 8;

        let result = calculator.multiply(n1, n2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn a_calculator_can_divide_correctly() {
        let calculator = Calculator::new();
        let n1 = 4;
        let n2 = 2;
        let expected_result = Ok(2);

        let result = calculator.divide(n1, n2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn try_divide_by_zero_should_give_error() {
        let calculator = Calculator::new();
        let n1 = 4;
        let n2 = 0;
        let expected_result = Err(CalculatorError::DivisionByZero.into());

        let result = calculator.divide(n1, n2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn a_calculator_sould_be_able_to_correctly_perform_the_request_operations() {
        let calculator = Calculator::new();
        let n1 = 4;
        let n2 = 2;
        let expected_addition_result = Ok(6);
        let expected_subtraction_result = Ok(2);
        let expected_multiplication_result = Ok(8);
        let expected_division_result = Ok(2);

        let addition_result = calculator.calculate(n1, n2, "+");
        let subtraction_result = calculator.calculate(n1, n2, "-");
        let multiplication_result = calculator.calculate(n1, n2, "*");
        let division_result = calculator.calculate(n1, n2, "/");

        assert_eq!(addition_result, expected_addition_result);
        assert_eq!(subtraction_result, expected_subtraction_result);
        assert_eq!(multiplication_result, expected_multiplication_result);
        assert_eq!(division_result, expected_division_result);
    }
}
