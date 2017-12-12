/// Type must implement this trait to get validated SP
/// RE - database connection acquire, IO, parsing errors
/// E - struct/map errors for user to display
/// T - underlying type after validation
pub trait ValidateSP<T, E, RE> {
    /// Either returns validated define type or expected errors
    fn validate(self) -> Result<SP<T, E>, RE>;
}

/// Returns struct params valid/invalid params
pub enum SP<T, E> {
    /// Valid type
    Valid(T),
    /// Invalid type
    Invalid(E),
}
