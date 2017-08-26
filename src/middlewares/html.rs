use iron::AfterMiddleware;
use iron::prelude::*;
use iron::headers::ContentType;

/// Middleware for injecting HTML content type into defined route
#[derive(Debug)]
pub struct Html;

impl AfterMiddleware for Html {
    fn after(&self, _req: &mut Request, mut resp: Response) -> IronResult<Response> {
        resp.headers.set(ContentType::html());
        Ok(resp)
    }
}
