use fastly::http::{StatusCode};
use fastly::{mime, Error, Request, Response};

const BACKEND_HTTPME: &str = "backend_httpme";

fn main() -> Result<(), Error> {
    let ds_req = Request::from_client();
    let us_resp = handler(ds_req)?;
    us_resp.send_to_client();
    Ok(())
}

fn handler(mut req: Request) -> Result<Response, Error> {
    // set the host header needed for glitch.
    req.set_header("host", "http-me.glitch.me");
    
    // Forward the request to a backend.
    let beresp = req.send(BACKEND_HTTPME)?;
    Ok(beresp)
}

#[test]
fn test_homepage() {
    let req = fastly::Request::get("http://http-me.edgecompute.app.com/");
    let resp = handler(req).expect("request succeeds");
    assert_eq!(resp.get_status(), StatusCode::OK);
    assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    assert!(resp.into_body_str().contains("Welcome to Compute@Edge"));
}
