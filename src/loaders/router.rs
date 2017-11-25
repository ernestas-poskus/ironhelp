use iron::Request;
use router::{Router, Params};

/// RouterLoader - gets Router from Iron Request
pub trait RouterLoader {
    /// router function to get Router Params
    fn router(&self) -> &Params;
}

impl<'a, 'b: 'a> RouterLoader for Request<'a, 'b> {
    /// router - method to get Router from Iron Request
    fn router(&self) -> &Params {
        self.extensions.get::<Router>().expect(
            "Router must be loaded by middleware",
        )
    }
}
