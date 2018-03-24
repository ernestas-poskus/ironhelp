use iron::AfterMiddleware;
use iron::prelude::*;
use iron::status;
use std::collections::HashMap;
use iron::headers::ContentType;

/// Middleware for catching HTTP status codes and responding with
/// appropriate message
pub struct StatusCatchMiddleware {
    /// Map of status codes with associated views
    pub map: HashMap<status::Status, (ContentType, String)>,
}

impl StatusCatchMiddleware {
    /// Initialize status catch middleware
    pub fn new() -> StatusCatchMiddleware {
        StatusCatchMiddleware {
            map: HashMap::new(),
        }
    }

    fn handle_status(&self, resp: &Response) -> Option<Response> {
        if let Some(ref status) = resp.status {
            if let Some(t) = self.map.get(status) {
                let mut r = Response::with((status.clone(), t.1.as_str()));
                r.headers.set(t.0.clone());

                Some(r)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl AfterMiddleware for StatusCatchMiddleware {
    fn after(&self, _req: &mut Request, resp: Response) -> IronResult<Response> {
        if let Some(res) = self.handle_status(&resp) {
            return Ok(res);
        }
        Ok(resp)
    }
}
