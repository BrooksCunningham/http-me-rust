use fastly::http::StatusCode;
use fastly::{Error, Request, Response};

const backend_httpme: &str = "backend_httpme";

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // set the host header needed for glitch.
    req.set_header("host", "http-me.glitch.me");
    
    // Forward the request to a backend.
    let mut beresp = req.send(backend_httpme)?;
    Ok(beresp)
}
