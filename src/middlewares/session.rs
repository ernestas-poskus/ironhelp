use iron::middleware::{BeforeMiddleware, AfterMiddleware};
use iron::headers::{Cookie as HeaderCookie, SetCookie};
use iron::{Request, Response, IronResult};
use iron::typemap;
use cookie::{CookieJar, Cookie};
use plugin::Extensible;
use iron::headers::Header;

/// Injects and retrieces CookieJar from request and
/// makes it available for in extensions
pub struct CookiesSession;

impl CookiesSession {
    /// Initialize cookies session for link middleware
    pub fn new() -> (CookiesSession, CookiesSession) {
        (CookiesSession, CookiesSession)
    }
}

impl typemap::Key for CookiesSession {
    type Value = CookieJar;
}

impl BeforeMiddleware for CookiesSession {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let mut jar: CookieJar = CookieJar::new();

        if let Some(cookies) = req.headers.get::<HeaderCookie>() {
            for cookie in cookies.iter() {
                match Cookie::parse(cookie.to_string()) {
                    Ok(c) => jar.add_original(c),
                    Err(e) => error!("Cookie::parse error {}", e),
                }
            }

            req.extensions.insert::<CookiesSession>(jar);
        } else {
            req.extensions.insert::<CookiesSession>(jar);
        }

        Ok(())
    }
}

impl AfterMiddleware for CookiesSession {
    fn after(&self, req: &mut Request, mut resp: Response) -> IronResult<Response> {
        for delta in req.cookies().delta() {
            resp.headers.append_raw(
                SetCookie::header_name(),
                delta.to_string().into_bytes(),
            );
        }

        Ok(resp)
    }
}

/// Convenience trait for retrieving cookies from Iron request
pub trait Session {
    /// Gets reference to cookies jar
    fn cookies(&self) -> &CookieJar;
    /// Gets mutable reference to cookies jar
    fn mut_cookies(&mut self) -> &mut CookieJar;
}

impl<'a, 'b: 'a> Session for Request<'a, 'b> {
    fn cookies(&self) -> &CookieJar {
        self.extensions().get::<CookiesSession>().expect(
            "CookiesSession not included",
        )
    }

    fn mut_cookies(&mut self) -> &mut CookieJar {
        self.extensions_mut().get_mut::<CookiesSession>().expect(
            "CookiesSession not included",
        )
    }
}
