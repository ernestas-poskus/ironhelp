#[macro_export]
macro_rules! response_ok {
    ($view:expr) => {{
        use iron::prelude::Response;
        Ok(Response::with((status::Ok, $view)))
    }}
}

#[macro_export]
macro_rules! response_201 {
    () => {{
        use iron::prelude::Response;
        use iron::status;
        Ok(Response::with((status::Created, "")))
    }}
}

#[macro_export]
macro_rules! response_202 {
    () => {{
        use iron::prelude::Response;
        use iron::status;
        Ok(Response::with((status::Accepted, "")))
    }}
}

#[macro_export]
macro_rules! response_204 {
    () => {{
        use iron::prelude::Response;
        use iron::status;
        Ok(Response::with((status::NoContent, "")))
    }}
}

#[macro_export]
macro_rules! redirect_301 {
    ($url:expr) => {{
        use iron::prelude::Response;
        use iron::modifiers::Redirect;
        use iron::status;
        Ok(Response::with((status::MovedPermanently, Redirect($url))))
    }}
}


#[macro_export]
macro_rules! redirect_303 {
    ($url:expr) => {{
        use iron::prelude::Response;
        use iron::modifiers::Redirect;
        use iron::status;
        Ok(Response::with((status::SeeOther, Redirect($url))))
    }}
}

#[macro_export]
macro_rules! response_400 {
    ($err:expr) => {{
        use iron::prelude::Response;
        use iron::status;
        Ok(Response::with((status::Unauthorized, $err)))
    }}
}

#[macro_export]
macro_rules! response_401 {
    () => {{
        use iron::prelude::Response;
        use iron::status;
        Ok(Response::with((status::Unauthorized, "")))
    }}
}

#[macro_export]
macro_rules! response_404 {
    ($err:expr) => {{
        use iron::prelude::Response;
        use iron::status;
        Ok(Response::with((status::NotFound, $err)))
    }}
}

#[macro_export]
macro_rules! response_500 {
    ($err:expr) => {{
        use iron::prelude::Response;
        use iron::status;
        Ok(Response::with((status::InternalServerError, $err)))
    }}
}
