pub const TRUE: i16 = -1;
pub const FALSE: i16 = 0;

#[derive(Debug, PartialEq)]
pub enum BooleanOperation {
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperation {
    LessThan,
    GreaterThan,
    Equal,
}

/// A struct to manage boolean operations.
/// It provides methods to execute boolean and logical operations.
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
    /// The result is `TRUE` if the operation is successful, otherwise `FALSE`.
    pub fn execute_boolean_operation(
        &mut self,
        op: &BooleanOperation,
        operand1: i16,
        operand2: Option<i16>,
    ) -> i16 {
        match op {
            BooleanOperation::And => {
                if operand1 == TRUE && operand2.unwrap_or(0) == TRUE {
                    TRUE
                } else {
                    FALSE
                }
            }
            BooleanOperation::Or => {
                if operand1 == TRUE || operand2.unwrap_or(0) == TRUE {
                    TRUE
                } else {
                    FALSE
                }
            }
            BooleanOperation::Not => {
                if operand1 == 0 {
                    TRUE
                } else {
                    FALSE
                }
            }
        }
    }

    /// Executes a logical operation on two operands.
    /// Returns the result of the operation as an `i16` value.
    /// The result is `TRUE` if the operation is successful, otherwise `FALSE`.
    /// The operations supported are:
    /// - LessThan (<)
    /// - GreaterThan (>)
    /// - Equal (=)  
    pub fn execute_logical_operations(
        &mut self,
        op: &LogicalOperation,
        operand1: i16,
        operand2: i16,
    ) -> i16 {
        match op {
            LogicalOperation::LessThan => {
                if operand1 < operand2 {
                    TRUE
                } else {
                    FALSE
                }
            }
            LogicalOperation::GreaterThan => {
                if operand1 > operand2 {
                    TRUE
                } else {
                    FALSE
                }
            }
            LogicalOperation::Equal => {
                if operand1 == operand2 {
                    TRUE
                } else {
                    FALSE
                }
            }
        }
    }

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
            manager.execute_boolean_operation(&BooleanOperation::And, TRUE, Some(TRUE)),
            TRUE
        );
        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::And, TRUE, Some(FALSE)),
            FALSE
        );
        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::Or, TRUE, Some(FALSE)),
            TRUE
        );
        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::Or, FALSE, Some(FALSE)),
            FALSE
        );
        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::Not, TRUE, None),
            FALSE
        );
        assert_eq!(
            manager.execute_boolean_operation(&BooleanOperation::Not, FALSE, None),
            TRUE
        );
    }

    #[test]
    fn can_execute_logical_operations() {
        let mut manager = BooleanOperationManager::new();

        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::LessThan, 1, 2),
            TRUE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::LessThan, 2, 1),
            FALSE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::GreaterThan, 2, 1),
            TRUE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::GreaterThan, 1, 2),
            FALSE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::Equal, 1, 1),
            TRUE
        );
        assert_eq!(
            manager.execute_logical_operations(&LogicalOperation::Equal, 1, 2),
            FALSE
        );
    }
}
