/// Constants for boolean operations
/// FORTH_TRUE and FORTH_FALSE are represented as i16 values.
/// FORTH_TRUE is -1 and FORTH_FALSE is 0.
/// This is a common convention in many programming languages.
/// The use of i16 allows for a wider range of values, but in this case,
/// we are only using -1 and 0 to represent FORTH_TRUE and FORTH_FALSE respectively.
pub const FORTH_TRUE: i16 = -1;
pub const FORTH_FALSE: i16 = 0;

/// Constants for boolean operations
pub const AND: BooleanOperation = BooleanOperation::And;
pub const OR: BooleanOperation = BooleanOperation::Or;
pub const NOT: BooleanOperation = BooleanOperation::Not;

/// Constants for logical operations
pub const LESS_THAN: LogicalOperation = LogicalOperation::LessThan;
pub const GREATER_THAN: LogicalOperation = LogicalOperation::GreaterThan;
pub const EQUAL: LogicalOperation = LogicalOperation::Equal;

/// Enumeration for boolean operations.
/// This enum defines the types of operations that can be performed.
/// The operations include:
/// - And (&&)
/// - Or (||)
/// - Not (!)
///     These operations are used to perform logical operations on boolean values.
#[derive(Debug, PartialEq)]
pub enum BooleanOperation {
    And,
    Or,
    Not,
}

/// Enumeration for logical operations.
/// This enum defines the types of logical operations that can be performed.
/// The operations include:
/// - LessThan (<)
/// - GreaterThan (>)
/// - Equal (=)
///     These operations are used to compare two values and return a boolean result.
#[derive(Debug, PartialEq)]
pub enum LogicalOperation {
    LessThan,
    GreaterThan,
    Equal,
}

/// A struct to manage boolean operations.
/// It provides methods to execute boolean and logical operations.
///
/// # Methods
/// - `execute_boolean_operation`: Executes a boolean operation on two operands.
/// - `execute_logical_operations`: Executes a logical operation on two operands.
/// - `is_not`: Checks if the operation is a NOT operation.
#[derive(Debug, PartialEq)]
pub struct BooleanOperationManager {}

impl Default for BooleanOperationManager {
    fn default() -> Self {
        BooleanOperationManager::new()
    }
}

impl BooleanOperationManager {
    /// Creates a new instance of the BooleanOperationManager.
    pub fn new() -> Self {
        BooleanOperationManager {}
    }

    /// Executes a boolean operation on two operands.
    /// The second operand is optional and defaults to 0 if not provided.
    /// Returns the result of the operation as an `i16` value.
    /// The result is `FORTH_TRUE` if the operation is successful, otherwise `FORTH_FALSE`.
    pub fn execute_boolean_operation(
        &mut self,
        operation: &BooleanOperation,
        op1: i16,
        op2: Option<i16>,
    ) -> i16 {
        match operation {
            BooleanOperation::And => {
                if op1 == FORTH_TRUE && op2.unwrap_or(0) == FORTH_TRUE {
                    FORTH_TRUE
                } else {
                    FORTH_FALSE
                }
            }
            BooleanOperation::Or => {
                if op1 == FORTH_TRUE || op2.unwrap_or(0) == FORTH_TRUE {
                    FORTH_TRUE
                } else {
                    FORTH_FALSE
                }
            }
            BooleanOperation::Not => {
                if op1 == 0 {
                    FORTH_TRUE
                } else {
                    FORTH_FALSE
                }
            }
        }
    }

    /// Executes a logical operation on two operands.
    /// Returns the result of the operation as an `i16` value.
    /// The result is `FORTH_TRUE` if the operation is successful, otherwise `FORTH_FALSE`.
    /// The operations supported are:
    /// - LessThan (<)
    /// - GreaterThan (>)
    /// - Equal (=)  
    pub fn execute_logical_operations(
        &mut self,
        operation: &LogicalOperation,
        op1: i16,
        op2: i16,
    ) -> i16 {
        match operation {
            LogicalOperation::LessThan => {
                if op1 < op2 {
                    FORTH_TRUE
                } else {
                    FORTH_FALSE
                }
            }
            LogicalOperation::GreaterThan => {
                if op1 > op2 {
                    FORTH_TRUE
                } else {
                    FORTH_FALSE
                }
            }
            LogicalOperation::Equal => {
                if op1 == op2 {
                    FORTH_TRUE
                } else {
                    FORTH_FALSE
                }
            }
        }
    }

    /// Checks if the operation is a NOT operation.
    pub fn is_not(&self, operation: &BooleanOperation) -> bool {
        matches!(operation, BooleanOperation::Not)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_execute_boolean_operation() {
        let mut manager = BooleanOperationManager::new();

        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::And, FORTH_TRUE, Some(FORTH_TRUE)),
            FORTH_TRUE
        );
        assert_eq!(
            manager.execute_boolean_operation(
                &BooleanOperation::And,
                FORTH_TRUE,
                Some(FORTH_FALSE)
            ),
            FORTH_FALSE
        );
        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::Or, FORTH_TRUE, Some(FORTH_FALSE)),
            FORTH_TRUE
        );
        assert_eq!(
            manager.execute_boolean_operation(
                &BooleanOperation::Or,
                FORTH_FALSE,
                Some(FORTH_FALSE)
            ),
            FORTH_FALSE
        );
        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::Not, FORTH_TRUE, None),
            FORTH_FALSE
        );
        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::Not, FORTH_FALSE, None),
            FORTH_TRUE
        );
    }

    #[test]
    fn can_execute_logical_operations() {
        let mut manager = BooleanOperationManager::new();

        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::LessThan, 1, 2),
            FORTH_TRUE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::LessThan, 2, 1),
            FORTH_FALSE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::GreaterThan, 2, 1),
            FORTH_TRUE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::GreaterThan, 1, 2),
            FORTH_FALSE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::Equal, 1, 1),
            FORTH_TRUE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::Equal, 1, 2),
            FORTH_FALSE
        );
    }
}
