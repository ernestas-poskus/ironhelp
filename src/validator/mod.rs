use std::error::Error as StdError;

/// Validator trait for validating anything what return standard error
pub trait Validator<E: StdError> {
    /// Validate anything what implements standard error
    fn validate(&self) -> Result<(), E>;
}
