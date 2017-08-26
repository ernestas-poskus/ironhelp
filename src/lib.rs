#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
//! Rust Iron framework helper macros & middleware's

extern crate iron;

use iron::{Chain, Handler};

/// All available middlewares
pub mod middlewares;
mod macros;

/// Injects JSON after middleware into given route(s)
/// and inserts json content type
pub fn content_type_json<H: Handler>(routes: H) -> Chain {
    let mut chain: Chain = Chain::new(routes);
    chain.link_after(middlewares::Json);
    chain
}

/// Injects HTML after middleware into given route(s)
/// and inserts html content type
pub fn content_type_html<H: Handler>(routes: H) -> Chain {
    let mut chain: Chain = Chain::new(routes);
    chain.link_after(middlewares::Html);
    chain
}