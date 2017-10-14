use iron::{AfterMiddleware, BeforeMiddleware, IronResult, IronError, Request, Response};
use iron::typemap::Key;
use std::time::Instant;

/// Logger middleware for logging request method, status, duration, url and IP address
pub struct LoggerMiddleware;

impl LoggerMiddleware {
    /// Logger initialise function
    pub fn new() -> (LoggerMiddleware, LoggerMiddleware) {
        (LoggerMiddleware, LoggerMiddleware)
    }
}

struct StartTime;
impl Key for StartTime {
    type Value = Instant;
}

impl LoggerMiddleware {
    fn initialise(&self, req: &mut Request) {
        req.extensions.insert::<StartTime>(Instant::now());
    }

    fn log(&self, req: &mut Request, res: &Response) -> IronResult<()> {
        let entry_time = *req.extensions.get::<StartTime>().unwrap();

        let response_time = entry_time.elapsed();
        let response_time_ms = (response_time.as_secs() * 1000) as f64 +
            (response_time.subsec_nanos() as f64) / 1000000.0;

        {
            let status = match res.status {
                Some(status) => status.to_u16(),
                None => 0,
            };
            let method = req.method.as_ref();
            let url = req.url.as_ref();
            let remote_addr = req.remote_addr;

            info!(
                "IP: {} Method: {}, Status: {}, Duration: {}ms, URL: {}",
                remote_addr,
                method,
                status,
                response_time_ms,
                url,
            )
        }

        Ok(())
    }
}

impl BeforeMiddleware for LoggerMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        self.initialise(req);
        Ok(())
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<()> {
        self.initialise(req);
        Err(err)
    }
}

impl AfterMiddleware for LoggerMiddleware {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        try!(self.log(req, &res));
        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        try!(self.log(req, &err.response));
        Err(err)
    }
}
