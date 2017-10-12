#[macro_export]
macro_rules! content_type_json {
    ($req:expr) => {{
        use iron::headers::ContentType;

        if let Some(ct) = $req.headers.get::<ContentType>() {
            ct == &ContentType::json()
        } else {
            false
        }
    }}
}

#[macro_export]
macro_rules! content_type_html {
    ($req:expr) => {{
        use iron::headers::ContentType;

        if let Some(ct) = $req.headers.get::<ContentType>() {
            ct == &ContentType::html()
        } else {
            false
        }
    }}
}
