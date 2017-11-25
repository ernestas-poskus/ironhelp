#[macro_export]
macro_rules! id_or_400 {
    ($req:expr) => {{
        use $crate::loaders::RouterLoader;
        use iron::headers::ContentType;

        match $req.router().find("id").unwrap_or("NONE").parse::<i64>() {
            Ok(n) => n,
            Err(_) => {
                if let Some(ct) = $req.headers.get::<ContentType>() {
                    if ct == &ContentType::json() {
                         return response_400!(r#"{"errors":{"id":"Please provide integer"}}"#);
                    }
                }
                return response_400!("Please provide integer")
            }
        }
    }}
}
