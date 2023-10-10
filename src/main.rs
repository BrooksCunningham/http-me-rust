use fastly::http::StatusCode;
use fastly::{Error, mime, KVStore, Request, Response};
use fastly::handle::client_ip_addr;
// use serde_json::{json, Value};

// const BACKEND_HTTPME: &str = "backend_httpme";

fn main() -> Result<(), Error> {
    let ds_req = Request::from_client();
    let us_resp = handler(ds_req)?;
    us_resp.send_to_client();
    Ok(())
}

fn handler(mut req: Request) -> Result<Response, Error> {
    // create a new response object that may be modified
    let mut resp = Response::new();

    resp = match req.get_header_str("endpoint") {
        Some(ep) if ep.contains("status") => status(&req, resp)?,
        _ => resp,
    };

    match req.get_path() {
        s if s.starts_with("/status") => return Ok(status(&req, resp)?),
        s if s.starts_with("/anything") => return Ok(anything(req, resp)?),
        s if s.starts_with("/static-assets/") => return Ok(get_static_asset(&req, resp)?),

        "/" => return Ok(swagger_ui_html(resp)?),
        
        // Do nothing
        _ => (),
    };
    return Ok::<fastly::Response, Error>(resp)
}

fn anything(mut req: Request, mut resp: Response) -> Result<Response, Error> {
    let mut reqHeadersData = serde_json::json!({});
    for (n, v) in req.get_headers() {
        let req_header_name_str = n.as_str();
        let req_header_val_str = v.to_str()?;
        reqHeadersData[req_header_name_str] = serde_json::json!(req_header_val_str);
    }
    // fastly::handle::client_ip_addr
    let client_ip_addr = client_ip_addr().unwrap().to_string();

    let req_url = req.get_url().to_owned();

    // https://developer.fastly.com/solutions/examples/manipulate-query-string/
    let qs = req_url.query().unwrap_or_else(|| "").to_string();
    let req_method = req.get_method_str().to_owned();

    let body = req.take_body_str();
    println!("{}", &body);

    let resp_data = serde_json::json!({
        "args": &qs,
        "body": &body,
        "headers": &reqHeadersData,
        "ip": &client_ip_addr,
        "method": &req_method,
        "url": &req_url.as_str(),
    });

    // resp.set_status(StatusCode::OK);
    resp.set_body_json(&resp_data);
    Ok(resp)
}

fn status(mut req: &Request, mut resp: Response) -> Result<Response, Error> {
    // let reqUrlAbs = Url::parse(req.get_url_str())?;
    let mut status_str = "";
    let mut statusParsed = 200;

    match req.get_header_str("endpoint") {
        Some(ep) if ep.contains("status") => {
            status_str = ep.split("=").collect::<Vec<&str>>()[1];
            statusParsed = status_str.parse::<u16>()?;
            return status_result(statusParsed, resp);
        },
        _ => ()
    }

    let req_url = req.get_url();
    let path_segments: Vec<&str> = req_url.path_segments().ok_or_else(|| "cannot be base").unwrap().collect();

    // If the path segment is too short, then just return a 500
    if path_segments.len() < 2 {
        resp.set_status(500);
        let data = serde_json::json!({ "error": "unable to parse status code properly. Try sending request like /status/302"});
        resp.set_body_json(&data);
        return Ok(resp);
    }

    status_str = path_segments[1];
    statusParsed = status_str.parse::<u16>()?;

    return status_result(statusParsed, resp);

    fn status_result(status_u16: u16, mut resp: Response) -> Result<Response, Error> {
        return match status_u16 {
            status_int => {
                // https://docs.rs/fastly/latest/fastly/http/struct.StatusCode.html
                resp.set_status(status_int);
                Ok(resp)
            },
            _ => {
                resp.set_status(500);
                let data = serde_json::json!({ "error": "unable to parse status code properly. Try sending request like /status/302"});
                resp.set_body_json(&data);
                Ok(resp)
            }
        }
    }
}

fn swagger_ui_html(mut resp: Response) -> Result<Response, Error> {
    // Define a KV store instance using the resource link name
  let store = KVStore::open("assets_store")?.unwrap();

  // Get the value back from the KV store (as a string),
  let swagger_html: String = store.lookup_str("static-assets/swagger.html")?.unwrap();

  resp.set_body_text_html(&swagger_html);
  return Ok(resp)
}

fn get_static_asset(req: &Request, mut resp: Response) -> Result<Response, Error> {

    let req_url = req.get_url();
    let path_segments: Vec<&str> = req_url.path_segments().ok_or_else(|| "cannot be base").unwrap().collect();

    let req_filename = path_segments.last().cloned().unwrap_or("Not Found");

    // Define a KV store instance using the resource link name
    let store = KVStore::open("assets_store")?.unwrap();

    // Get the value back from the KV store (as a string),
    let req_filename_lookup = format!("static-assets/{}", &req_filename);
    let static_asset: String = store.lookup_str(&req_filename_lookup)?.unwrap_or("Not Found".to_string());

    // using the set_body_text_plain since that accepts a &str value.
    resp.set_body_text_plain(&static_asset);
    
    let filename_parts = req_filename.split(".").collect::<Vec<&str>>();
    let filename_ext = filename_parts.last().cloned().unwrap_or("html");

    match filename_ext {
        "js" => resp.set_header("content-type", "application/javascript; charset=utf-8"),
        "css" => resp.set_header("content-type", "text/css; charset=utf-8"),
        "html" => resp.set_header("content-type", "text/html; charset=utf-8"),
        "json" => resp.set_header("content-type", "application/json; charset=utf-8"),
        _ => resp.set_body_text_plain(&static_asset),
    };

    return Ok(resp)
}


#[test]
fn test_homepage() {
    let req = fastly::Request::get("http://http-me.edgecompute.app.com/");
    let resp = handler(req).expect("request succeeds");
    assert_eq!(resp.get_status(), StatusCode::OK);
    assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    assert!(resp.into_body_str().contains("Welcome to Compute@Edge"));
}
