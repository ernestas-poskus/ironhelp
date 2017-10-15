/// Type must implement this trait to get validated StructParams
/// RE - database connection acquire, IO, parsing errors
/// E - struct/map errors for user to display
/// T - underlying type after validation
pub trait ValidateStructParams<T, E, RE> {
    /// Either returns validated define type or expected errors
    fn validate(&self) -> Result<StructParams<T, E>, RE>;
}

/// Returns valid/invalid params
pub enum StructParams<T, E> {
    /// Valid type
    Valid(T),
    /// Invalid type
    Invalid(E),
}
