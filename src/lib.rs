extern crate iron;

use iron::{Chain, Handler};

// local
pub mod middlewares;
mod macros;

pub fn content_type_json<H: Handler>(routes: H) -> Chain {
    let mut chain: Chain = Chain::new(routes);
    chain.link_after(middlewares::Json);
    chain
}

pub fn content_type_html<H: Handler>(routes: H) -> Chain {
    let mut chain: Chain = Chain::new(routes);
    chain.link_after(middlewares::Html);
    chain
}
