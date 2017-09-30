#[macro_export]
macro_rules! request_validate {
    ($req:expr, $errors:expr) => {{
        use iron::{IronError, status};

        Err(IronError::new($errors, status::BadRequest))
    }}
}
