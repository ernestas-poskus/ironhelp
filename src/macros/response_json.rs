#[macro_export]
macro_rules! response_json_ok {
    ($view:expr) => {{
        use iron::prelude::Response;
        use iron::status;
        use serde_json;
        use iron::headers::ContentType;

        let mut resp = Response::with((
            status::Ok,
            serde_json::to_string(&$view).expect("response_json_ok! Expected valid JSON")
        ));
        resp.headers.set(ContentType::json());
        Ok(resp)
    }}
}

#[macro_export]
macro_rules! response_json_400 {
    ($view:expr) => {{
        use iron::prelude::Response;
        use iron::status;
        use serde_json;
        use iron::headers::ContentType;

        let mut resp = Response::with((
            status::BadRequest,
            serde_json::to_string(&$view).expect("response_json_400! Expected valid JSON")
        ));
        resp.headers.set(ContentType::json());
        Ok(resp)
    }}
}
