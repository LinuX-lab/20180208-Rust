use std::path::Path;
use iron::prelude::*;
use iron::{Iron,AroundMiddleware, Handler};
use mount::Mount;
use staticfile::Static;

struct Logger {}

struct LoggerHandler<H: Handler> {
    logger: Logger,
    handler: H,
}

impl Logger {
    fn new() -> Logger {
        Logger {}
    }

    fn log(&self, req: &Request, res: Result<&Response, &IronError>, time: u64) {
        println!(
            "{} {} {:?} {}us",
            req.method,
            req.url,
            match res {
                Ok(x) => x.status.unwrap(),
                Err(x) => x.response.status.unwrap(),
            },
            time
        );
    }
}

impl<H: Handler> Handler for LoggerHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let entry = ::time::precise_time_ns();
        let res = self.handler.handle(req);
        self.logger
            .log(req, res.as_ref(), ::time::precise_time_ns() - entry);
        res
    }
}

impl AroundMiddleware for Logger {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(LoggerHandler {
            logger: self,
            handler,
        }) as Box<Handler>
    }
}

fn main() {
    let mut mount = Mount::new();

    // Serve the shared JS/CSS at /
    mount.mount(
        "/",
        Logger::new().around(Box::new(Static::new(Path::new(".")))),
    );

    println!("Server running on http://localhost:3000/");

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
