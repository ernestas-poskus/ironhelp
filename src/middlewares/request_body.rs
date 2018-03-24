use iron::middleware::BeforeMiddleware;
use iron::{method, status, typemap, IronError, IronResult, Request};
use plugin::Extensible;
use std::io::Read;
use std::error::Error;
use std::fmt::{self, Formatter};

/// RequestBodyError defines errors for too large payload
/// If request method is: GET, HEAD, OPTIONS validation is skipped
#[derive(Debug)]
pub enum RequestBodyError {
    /// PayloadTooLarge - request payload is too large
    PayloadTooLarge,
}

impl fmt::Display for RequestBodyError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.description().fmt(fmt)
    }
}

impl Error for RequestBodyError {
    fn description(&self) -> &str {
        "Request entity is larger than limits defined by server"
    }
}

/// RequestBodyMiddleware verifies payload is not too large
pub struct RequestBodyMiddleware(usize);

impl RequestBodyMiddleware {
    /// Initialize RequestBodyMiddleware with a request payload limit in bytes
    pub fn new(size_in_bytes: usize) -> RequestBodyMiddleware {
        RequestBodyMiddleware(size_in_bytes)
    }
}

impl typemap::Key for RequestBodyMiddleware {
    type Value = Vec<u8>;
}

fn skip_method(m: &method::Method) -> bool {
    return m == &method::Get || m == &method::Head || m == &method::Options;
}

impl BeforeMiddleware for RequestBodyMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        if skip_method(&req.method) {
            return Ok(());
        }

        let mut body = Vec::new();
        match req.body.read_to_end(&mut body) {
            Ok(size_read) => {
                if self.0 < size_read {
                    return Err(IronError::new(
                        RequestBodyError::PayloadTooLarge,
                        (
                            status::PayloadTooLarge,
                            format!("Payload too large, must be at most {} bytes", self.0),
                        ),
                    ));
                }

                req.extensions.insert::<RequestBodyMiddleware>(body);
                Ok(())
            }

            Err(e) => Err(IronError::new(e, status::InternalServerError)),
        }
    }
}

/// Convenience trait for retrieving Request body from Iron request
pub trait RequestBody {
    /// Gets reference to buffered Request body
    /// panic's if called on skipped request method: GET, HEAD, OPTIONS
    fn request_body(&self) -> &Vec<u8>;
}

impl<'a, 'b: 'a> RequestBody for Request<'a, 'b> {
    fn request_body(&self) -> &Vec<u8> {
        self.extensions()
            .get::<RequestBodyMiddleware>()
            .expect("RequestBodyMiddleware not included")
    }
}
