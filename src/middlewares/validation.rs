use iron::{AfterMiddleware, IronResult, Request, Response, IronError, status};
use validation::ValidationError;

/// ValidationMiddleware responds with BadRequest and serialized
/// validation errors
pub struct ValidationMiddleware;

impl AfterMiddleware for ValidationMiddleware {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(e) = err.error.downcast_ref::<ValidationError>() {
            return Ok(Response::with((status::BadRequest, e.to_string())));
        }
        Err(err)
    }
}
