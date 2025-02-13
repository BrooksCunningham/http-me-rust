// use core::fmt;
use fastly::handle::client_ip_addr;
// #[allow(unused_imports)]
use fastly::http::{HeaderValue, Method, StatusCode};
use fastly::Body;
use fastly::{Backend, Error, KVStore, Request, Response};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() -> Result<(), Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    let client_req = Request::from_client();

    let mut server_resp = handler(client_req)?;

    match server_resp.get_header_str("action-tarpit") {
        Some(ep) if ep.contains("1") => {
            let body = server_resp.take_body();
            let mut streamer = server_resp.stream_to_client();
            // The following code will force the client to wait for 1 second
            // before emitting each 100 bytes of the response.
            for chunk in body.into_bytes().as_slice().chunks(100) {
                let _ = streamer.write(chunk)?;
                streamer.flush()?;
                sleep(Duration::from_millis(1000));
            }
            return Ok(());
        }
        _ => (),
    };

    server_resp.send_to_client();
    Ok(())
}

#[allow(unused_mut)]
fn handler(mut req: Request) -> Result<Response, Error> {
    // create a new response object that may be modified
    let mut resp = Response::new();

    resp = match req.get_header_str("endpoint") {
        Some(ep) if ep.contains("status") => status(&req, resp)?,
        _ => resp,
    };

    resp = match req.get_query_parameter("status") {
        Some(ep) => {
            println!("{}", ep);
            status(&req, resp)?
        }
        _ => resp,
    };

    // tarpit implementation
    // https://github.com/BrooksCunningham/Fastly-Training-Demos/blob/d35589eb6652c9f8df29e407d4a6177f11c5ff7a/TarPit/src/main.rs#L27

    // TODOs
    // Add do tarpitting in the response header in the handler function if tarpitting should occur. Get that header and tarpit based on some information in the main function.
    match req.get_header_str("endpoint") {
        Some(ep) if ep.contains("tarpit") => resp.set_header("action-tarpit", "1"),
        _ => (),
    };

    match req.get_path() {
        s if s.starts_with("/status") => return Ok(status(&req, resp)?),
        s if s.starts_with("/anything") => return Ok(anything(req, resp)?),
        s if s.starts_with("/static-assets/") => return Ok(get_static_asset(&req, resp)?),
        s if s.starts_with("/forms/post") => return Ok(get_static_asset(&req, resp)?),
        s if s.starts_with("/dynamic_backend") => return Ok(dynamic_backend(req, resp)?),
        s if s.starts_with("/client_ip_data") => return Ok(client_ip_data(req, resp)?),

        "/" => return Ok(swagger_ui_html(resp)?),

        // Do nothing
        _ => (),
    };
    return Ok::<fastly::Response, Error>(resp);
}

// Define a struct to deserialize the incoming JSON body
#[derive(Deserialize)]
struct ClientDynamicBackendRequestBody {
    backend: String,
    method: Option<String>,
    target_url: Option<String>,
    headers: Option<Value>,
    repeat: Option<u64>,
}

fn dynamic_backend(mut req: Request, _resp: Response) -> Result<Response, Error> {
    // Start timing the request processing
    let start = Instant::now();

    // Parse the JSON body from the incoming request
    let body: ClientDynamicBackendRequestBody =
        match req.take_body_json::<ClientDynamicBackendRequestBody>() {
            Ok(b) => b,
            Err(_) => {
                // println!("{:?}", e);
                return Ok(Response::from_status(400).with_body("Invalid JSON"));
            }
        };

    // Extract backend, headers, and repeat values from the parsed body
    let target_host = body.backend;
    let target_url = body.target_url.unwrap_or(format!("{}", &target_host));
    let repeat = body.repeat.unwrap_or(1);
    let headers = body.headers.unwrap_or(json!({}));
    let target_method = body.method.unwrap_or("GET".to_string());

    // Dynamic backend builder
    let target_backend = Backend::builder(&target_host, &target_host)
        .override_host(&target_host)
        .connect_timeout(Duration::from_secs(1))
        .first_byte_timeout(Duration::from_secs(15))
        .between_bytes_timeout(Duration::from_secs(10))
        .enable_ssl()
        .sni_hostname(&target_host)
        .finish()?;

    // Initialize a response object to store the final response
    let mut final_response = Response::new();

    // Create a new backend request
    let mut backend_req_builder = Request::new(target_method, format!("https://{}", target_url));

    // Add custom headers to the backend request
    println!("DEBUG {}", headers);
    if let Some(headers_obj) = headers.as_object() {
        for (header_name, header_value) in headers_obj {
            let header_value_str = header_value.to_owned().take();
            println!("{}:{}", header_name, header_value_str);
            backend_req_builder.set_header(header_name, header_value_str.to_string());

            // backend_req_builder.set_header(
            //     header_name,
            //     HeaderValue::from_str(header_value.as_str().unwrap_or("")).unwrap(),
            // );
        }
    }

    // Repeat the request the specified number of times
    for i in 0..repeat {
        // Clone the previously built request
        let backend_req = backend_req_builder.clone_with_body();

        println!("{:?}", backend_req.get_header("bypass-waf"));

        // Send the request to the backend
        let backend_resp = backend_req.send(&target_backend)?;

        if i + 1 == repeat {
            // Append the backend response to the final response
            final_response.set_status(backend_resp.get_status());
            final_response.set_body(backend_resp.into_body());
        }
    }

    // Calculate the elapsed time and set it as a response header
    let duration = start.elapsed().as_millis();
    final_response.set_header(
        "response-timing",
        HeaderValue::from_str(&duration.to_string()).unwrap(),
    );

    // Return the final response
    Ok(final_response)
}

fn anything(mut req: Request, mut resp: Response) -> Result<Response, Error> {
    let mut req_headers_data: Value = serde_json::json!({});
    for (n, v) in req.get_headers() {
        let req_header_name_str = n.as_str();
        let req_header_val_str = v.to_str()?;
        req_headers_data[req_header_name_str] = serde_json::json!(req_header_val_str);
    }
    // fastly::handle::client_ip_addr
    let client_ip_addr: String = client_ip_addr().unwrap().to_string();

    let req_url = req.get_url().to_owned();

    // https://developer.fastly.com/solutions/examples/manipulate-query-string/
    let qs = req_url.query().unwrap_or_else(|| "").to_string();
    let req_method = req.get_method_str().to_owned();

    // let body = req.take_body_str();
    let buffer = req.take_body_bytes();
    let body = String::from_utf8_lossy(&buffer);

    let resp_data = serde_json::json!({
        "args": &qs,
        "body": &body,
        "headers": &req_headers_data,
        "ip": &client_ip_addr,
        "method": &req_method,
        "url": &req_url.as_str(),
    });

    let _ = resp.set_body_json(&resp_data);
    Ok(resp)
}

fn status(req: &Request, mut resp: Response) -> Result<Response, Error> {
    let mut status_str = "";
    let mut status_parsed = 200;

    match req.get_header_str("endpoint") {
        Some(ep) if ep.contains("status") => {
            status_str = ep.split("=").collect::<Vec<&str>>()[1];
            status_parsed = status_str.parse::<u16>()?;
            return status_result(status_parsed, resp);
        }
        _ => (),
    }

    match req.get_query_parameter("status") {
        Some(ep) => {
            status_parsed = ep.parse::<u16>()?;
            return status_result(status_parsed, resp);
        }
        _ => (),
    }

    let req_url = req.get_url();
    let path_segments: Vec<&str> = req_url
        .path_segments()
        .ok_or_else(|| "cannot be base")
        .unwrap()
        .collect();

    // If the path segment is too short, then just return a 500
    if path_segments.len() < 2 {
        resp.set_status(500);
        let data = serde_json::json!({ "error": "unable to parse status code properly. Try sending request like /status/302"});
        let _ = resp.set_body_json(&data);
        return Ok(resp);
    }

    status_str = path_segments[1];
    status_parsed = status_str.parse::<u16>()?;

    return status_result(status_parsed, resp);
}

fn status_result(status_u16: u16, mut resp: Response) -> Result<Response, Error> {
    return match status_u16 {
        status_int => {
            // https://docs.rs/fastly/latest/fastly/http/struct.StatusCode.html
            resp.set_status(status_int);
            Ok(resp)
        }
        _ => {
            resp.set_status(500);
            let data = serde_json::json!({ "error": "unable to parse status code properly. Try sending request like /status/302"});
            let _ = resp.set_body_json(&data);
            Ok(resp)
        }
    };
}

fn swagger_ui_html(mut resp: Response) -> Result<Response, Error> {
    // Define a KV store instance using the resource link name
    let store: KVStore = KVStore::open("assets_store")?.unwrap();

    // Get the value back from the KV store (as a string)
    let swagger_html: Body  = store.lookup("static-assets/swagger.html")?.take_body();

    resp.set_body(swagger_html);
    return Ok(resp);
}

fn get_static_asset(req: &Request, mut resp: Response) -> Result<Response, Error> {
    let req_url = req.get_url();
    let path_segments: Vec<&str> = req_url
        .path_segments()
        .ok_or_else(|| "cannot be base")
        .unwrap()
        .collect();

    let req_filename = path_segments.last().cloned().unwrap_or("Not Found");

    // Define a KV store instance using the resource link name
    let store = KVStore::open("assets_store")?.unwrap();

    // Get the value back from the KV store (as a string),
    // let req_filename_lookup = format!("static-assets/{}", &req_filename);
    // let static_asset: Body  = store.lookup(&req_filename_lookup)?.take_body();
    let static_asset: Body  = store.lookup(&req_filename)?.take_body();

    let static_filename_parts = req_filename.split(".").collect::<Vec<&str>>();
    let static_filename_ext = static_filename_parts.last().cloned().unwrap_or("html");

    match static_filename_ext {
        "js" => resp.set_header("content-type", "application/javascript; charset=utf-8"),
        "css" => resp.set_header("content-type", "text/css; charset=utf-8"),
        "html" => resp.set_header("content-type", "text/html; charset=utf-8"),
        "json" => resp.set_header("content-type", "application/json; charset=utf-8"),
        "jpg" => resp.set_header("content-type", "image/jpg"),
        "png" => resp.set_header("content-type", "image/png"),
        _ => resp.set_header("content-type", "text/plain"),
    };

    resp.set_body(static_asset);

    return Ok(resp);
}

fn client_ip_data(req: Request, mut resp: Response) -> Result<Response, Error> {
    // Attempt to get the 'ip' query parameter.
    // If present, try to parse it as an IpAddr; if that fails, or if not present,
    // use the client's actual IP address.
    let ip_addr: std::net::IpAddr = if let Some(ip_param) = req.get_query_parameter("ip") {
        match ip_param.parse() {
            Ok(parsed_ip) => parsed_ip,
            _ => req.get_client_ip_addr().unwrap(), // fallback to client's IP on parse error
        }
    } else {
        req.get_client_ip_addr().unwrap()
    };

    // Use geo_lookup to get the Geo object based on the chosen IP address.
    let geo_data: fastly::geo::Geo = fastly::geo::geo_lookup(ip_addr).unwrap();
    
    // Dynamically build the JSON object with the geo lookup results.
    let json_data = json!({
        "ip_address": ip_addr.to_string(),
        "as_name": geo_data.as_name(),
        "as_number": geo_data.as_number(),
        "area_code": geo_data.area_code(),
        "city": geo_data.city(),
        "conn_speed": geo_data.conn_speed(),
        "conn_type": geo_data.conn_type(),
        "continent": geo_data.continent(),
        "country_code": geo_data.country_code(),
        "country_name": geo_data.country_name(),
        "latitude": geo_data.latitude(),
        "longitude": geo_data.longitude(),
        "metro_code": geo_data.metro_code(),
        "postal_code": geo_data.postal_code(),
        "proxy_description": geo_data.proxy_description(),
        "proxy_type": geo_data.proxy_type(),
        "region": geo_data.region()
    });

    // Set the JSON body of the response.
    let _ = resp.set_body_json(&json_data);
    Ok(resp)
}

// #[test]
// fn test_homepage() {
//     let req: Request = fastly::Request::get("http://http-me.edgecompute.app.com/");
//     let resp: Response = handler(req).expect("request succeeds");
//     assert_eq!(resp.get_status(), StatusCode::OK);
//     assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
//     assert!(resp.into_body_str().contains("Welcome to Compute@Edge"));
// }
