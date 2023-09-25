use fastly::http::{StatusCode};
use fastly::{Error, Request, Response};
use fastly::handle::client_ip_addr;
// use serde_json::{json, Value};
use serde_json::json;

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

    match req.get_path() {
        "/anything2/1" => {
            Ok(anything(req)?)
        }
        s if s.starts_with("/anything/") => {
            Ok(anything(req)?)
        }
        // Forward the request to a backend.
        _ => {
            let beresp = req.send(BACKEND_HTTPME)?;
            Ok(beresp)
        }
    }
}

fn anything(mut req: Request) -> Result<Response, Error> {
    let mut reqHeadersData = serde_json::json!({});
    for (n, v) in req.get_headers() {
        let reqHeaderNameStr = n.as_str();
        let reqHeaderValStr = v.to_str()?;
        // println!("Header -  {}: {}", n, reqHeaderStr);
        reqHeadersData[reqHeaderNameStr] = serde_json::json!(reqHeaderValStr);
    }
    // fastly::handle::client_ip_addr
    let client_ip_addr = client_ip_addr().unwrap().to_string();
    println!("{:?}", &client_ip_addr);


    let mut resp_data = serde_json::json!({
        "ip": &client_ip_addr,
        "method": "somemethod",
        "args": {},
        "headers": reqHeadersData,
        "url": "",
    });
    println!("{:?}",&resp_data);
    let mut resp = Response::new();
    resp.set_status(StatusCode::OK);
    resp.set_body_json(&resp_data);
    Ok(resp)
}

#[test]
fn test_homepage() {
    let req = fastly::Request::get("http://http-me.edgecompute.app.com/");
    let resp = handler(req).expect("request succeeds");
    assert_eq!(resp.get_status(), StatusCode::OK);
    assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    assert!(resp.into_body_str().contains("Welcome to Compute@Edge"));
}
