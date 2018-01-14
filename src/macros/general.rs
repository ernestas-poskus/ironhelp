#[macro_export]
macro_rules! log_on_error {
    ($e:expr) => {{
        match $e {
            Ok(_) => {},
            Err(err) => {
                error!("log_on_error {}", err);
            }
        }
    }}
}
