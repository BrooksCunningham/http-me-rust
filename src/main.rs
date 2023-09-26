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
        s if s.starts_with("/status") => {
            Ok(status(req)?)
        }
        s if s.starts_with("/anything") => {
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
        reqHeadersData[reqHeaderNameStr] = serde_json::json!(reqHeaderValStr);
    }
    // fastly::handle::client_ip_addr
    let client_ip_addr = client_ip_addr().unwrap().to_string();

    let reqUrl = req.get_url().to_owned();

    // https://developer.fastly.com/solutions/examples/manipulate-query-string/
    let qs = reqUrl.query().unwrap_or_else(|| "").to_string();
    let req_method = req.get_method_str().to_owned();

    let body = req.take_body_str();
    println!("{}", &body);

    let resp_data = serde_json::json!({
        "args": &qs,
        "body": &body,
        "headers": &reqHeadersData,
        "ip": &client_ip_addr,
        "method": &req_method,
        "url": &reqUrl.as_str(),
    });

    let mut resp = Response::new();
    resp.set_status(StatusCode::OK);
    resp.set_body_json(&resp_data);
    Ok(resp)
}

fn status(req: Request) -> Result<Response, Error> {
    // let reqUrlAbs = Url::parse(req.get_url_str())?;
    let reqUrl = req.get_url();
    println!("{:?}", reqUrl);
    // println!("{:?}", reqUrl.path_segments());
    // let mut path_segments = reqUrl.path_segments().ok_or_else(|| "cannot be base")?;
    println!();
    let path_segments: Vec<&str> = reqUrl.path_segments().ok_or_else(|| "cannot be base").unwrap().collect();

    let status_str = path_segments[1];
    let statusResult = status_str.parse::<u16>();
    println!("{:?}", statusResult);

    let mut resp = Response::new();
    match statusResult {
        Ok(statusInt) => {
            // https://docs.rs/fastly/latest/fastly/http/struct.StatusCode.html
            resp.set_status(statusInt);
            return Ok(resp);
        }
        Err(_) => {
            resp.set_status(500);
            let data = serde_json::json!({ "error": "unable to parse status code properly. Try sending request like /status/302"});
            resp.set_body_json(&data);
            return Ok(resp);
        }
    }
}

#[test]
fn test_homepage() {
    let req = fastly::Request::get("http://http-me.edgecompute.app.com/");
    let resp = handler(req).expect("request succeeds");
    assert_eq!(resp.get_status(), StatusCode::OK);
    assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    assert!(resp.into_body_str().contains("Welcome to Compute@Edge"));
}
