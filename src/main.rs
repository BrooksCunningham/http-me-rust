//! # HTTP-ME: HTTP Testing Service
//!
//! A Fastly Compute@Edge service for HTTP testing and debugging.
//!
//! ## Features
//!
//! - **Status Code Testing**: Return any HTTP status code via `/status/{code}`
//! - **Request Inspection**: Echo back request data via `/anything/{path}`
//! - **Static Assets**: Serve static files and Swagger UI documentation
//! - **Client IP Data**: Get geolocation data for client IPs via `/client_ip_data`
//! - **Header Utilities**: Set custom response headers via `/utilities/set_headers`
//! - **Tarpit**: Simulate slow responses for testing timeouts
//!
//! ## Example Usage
//!
//! ```bash
//! # Get a 404 status code
//! curl -i https://http-me.edgecompute.app/status/404
//!
//! # Echo request data
//! curl https://http-me.edgecompute.app/anything/test?foo=bar
//!
//! # Set custom status via header
//! curl -H 'endpoint:status=302' https://http-me.edgecompute.app/any/path
//! ```

use fastly::handle::client_ip_addr;
use fastly::http::{Method, StatusCode};
use fastly::Body;
use fastly::{Error, KVStore, Request, Response};
use serde_json::json;
use serde_json::Value;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

// Constants for configuration and magic values
const KV_STORE_NAME: &str = "assets_store";
const SWAGGER_HTML_KEY: &str = "swagger.html";
const COMPRESS_HINT_HEADER: &str = "x-compress-hint";
const TARPIT_ACTION_HEADER: &str = "action-tarpit";
const ENDPOINT_HEADER: &str = "endpoint";
const STATUS_QUERY_PARAM: &str = "status";
const IP_QUERY_PARAM: &str = "ip";
const TARPIT_CHUNK_SIZE: usize = 100;
const TARPIT_DELAY_MS: u64 = 1000;
const CUSTOM_HEADER_PREFIX: &str = "x-";
const RESPONSE_HEADER_PREFIX: &str = "resp-";
const DEFAULT_STATUS_CODE: u16 = 200;
const ERROR_STATUS_CODE: u16 = 500;

// Path prefixes for routing
const PATH_STATUS: &str = "/status";
const PATH_ANYTHING: &str = "/anything";
const PATH_STATIC_ASSETS: &str = "/static-assets/";
const PATH_FORMS_POST: &str = "/forms/post";
const PATH_SET_HEADERS: &str = "/utilities/set_headers";
const PATH_CLIENT_IP: &str = "/client_ip_data";
const PATH_ROOT: &str = "/";

// CORS headers
const CORS_ALLOW_ORIGIN: &str = "*";
const CORS_ALLOW_METHODS: &str = "GET, POST, PUT, DELETE, OPTIONS";
const CORS_ALLOW_HEADERS: &str = "Content-Type, Authorization";

/// Entry point for the Fastly Compute@Edge service.
///
/// Processes incoming client requests and handles special features like tarpit mode
/// for simulating slow responses.
///
/// # Errors
///
/// Returns an error if request handling or streaming fails.
fn main() -> Result<(), Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    let client_req = Request::from_client();

    let mut server_resp = handler(client_req)?;
    // https://www.fastly.com/documentation/guides/concepts/compression/#dynamic-compression
    server_resp.set_header(COMPRESS_HINT_HEADER, "on");

    match server_resp.get_header_str(TARPIT_ACTION_HEADER) {
        Some(ep) if ep.contains("1") => {
            let body = server_resp.take_body();
            let mut streamer = server_resp.stream_to_client();
            // The following code will force the client to wait for 1 second
            // before emitting each 100 bytes of the response.
            for chunk in body.into_bytes().as_slice().chunks(TARPIT_CHUNK_SIZE) {
                let _ = streamer.write(chunk)?;
                streamer.flush()?;
                sleep(Duration::from_millis(TARPIT_DELAY_MS));
            }
            return Ok(());
        }
        _ => (),
    };

    server_resp.send_to_client();
    Ok(())
}

/// Main request handler that routes requests to appropriate endpoint handlers.
///
/// Routes are matched based on:
/// - Custom headers (e.g., `endpoint:status=302`)
/// - Query parameters (e.g., `?status=404`)
/// - URL paths (e.g., `/status/200`, `/anything/test`)
///
/// # Arguments
///
/// * `req` - The incoming HTTP request
///
/// # Returns
///
/// Returns a `Response` object with the appropriate content and status code.
///
/// # Errors
///
/// Returns an error if any of the endpoint handlers fail.
#[allow(unused_mut)]
fn handler(mut req: Request) -> Result<Response, Error> {
    // create a new response object that may be modified
    let mut resp = Response::new();

    resp = match req.get_header_str(ENDPOINT_HEADER) {
        Some(ep) if ep.contains("status") => status(&req, resp)?,
        _ => resp,
    };

    resp = match req.get_query_parameter(STATUS_QUERY_PARAM) {
        Some(ep) => {
            println!("{}", ep);
            status(&req, resp)?
        }
        _ => resp,
    };

    // tarpit implementation
    // https://github.com/BrooksCunningham/Fastly-Training-Demos/blob/d35589eb6652c9f8df29e407d4a6177f11c5ff7a/TarPit/src/main.rs#L27

    // Add tarpitting header in the response if tarpitting should occur. 
    // The main function checks this header and applies tarpit based on its value.
    match req.get_header_str(ENDPOINT_HEADER) {
        Some(ep) if ep.contains("tarpit") => resp.set_header(TARPIT_ACTION_HEADER, "1"),
        _ => (),
    };

    match req.get_path() {
        s if s.starts_with(PATH_STATUS) => return status(&req, resp),
        s if s.starts_with(PATH_ANYTHING) => return anything(req, resp),
        s if s.starts_with(PATH_STATIC_ASSETS) => return get_static_asset(&req, resp),
        s if s.starts_with(PATH_FORMS_POST) => return get_static_asset(&req, resp),
        // s if s.starts_with("/dynamic_backend") => return dynamic_backend(req, resp),
        s if s.starts_with(PATH_SET_HEADERS) => return set_headers(req, resp),
        s if s.starts_with(PATH_CLIENT_IP) => return client_ip_data(req, resp),

        PATH_ROOT => return swagger_ui_html(resp),

        // Do nothing
        _ => (),
    };
    Ok::<fastly::Response, Error>(resp)
}

/// Echoes back all request data including headers, body, query string, and client IP.
///
/// This endpoint is useful for debugging HTTP requests and inspecting what data
/// is being sent to the server.
///
/// # CORS Support
///
/// This endpoint handles CORS preflight requests (OPTIONS) and sets appropriate
/// CORS headers to allow cross-origin requests.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request
/// * `resp` - The response object to populate
///
/// # Returns
///
/// Returns a JSON response containing:
/// - `args`: Query string parameters
/// - `body`: Request body content
/// - `headers`: All request headers
/// - `ip`: Client IP address
/// - `method`: HTTP method used
/// - `url`: Full request URL
///
/// # Errors
///
/// Returns an error if JSON serialization or header parsing fails.
fn anything(mut req: Request, mut resp: Response) -> Result<Response, Error> {
    // Handle OPTIONS requests for CORS preflight
    if req.get_method() == Method::OPTIONS {
        return Ok(Response::from_status(StatusCode::OK)
            .with_header("Access-Control-Allow-Origin", CORS_ALLOW_ORIGIN)
            .with_header("Access-Control-Allow-Methods", CORS_ALLOW_METHODS)
            .with_header("Access-Control-Allow-Headers", CORS_ALLOW_HEADERS)
            // .with_header("Access-Control-Max-Age", "86400") // 24 hours.  Good practice, helps performance.
            .with_body(Body::new())); // Important:  Empty body for the preflight response.
    }
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
    let qs = req_url.query().unwrap_or("").to_string();
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
    resp.set_header("Access-Control-Allow-Origin", CORS_ALLOW_ORIGIN);
    Ok(resp)
}

/// Returns a response with a specific HTTP status code.
///
/// The status code can be specified in multiple ways (in order of precedence):
/// 1. Custom header: `endpoint:status=404`
/// 2. Query parameter: `?status=404`
/// 3. URL path segment: `/status/404`
///
/// # Arguments
///
/// * `req` - The incoming HTTP request
/// * `resp` - The response object to modify
///
/// # Returns
///
/// Returns a response with the requested status code.
///
/// # Errors
///
/// Returns a 500 error if the status code cannot be parsed from the path.
fn status(req: &Request, mut resp: Response) -> Result<Response, Error> {
    match req.get_header_str(ENDPOINT_HEADER) {
        Some(ep) if ep.contains("status") => {
            let status_str = ep.split('=').collect::<Vec<&str>>()[1];
            let status_parsed = status_str.parse::<u16>().unwrap_or(DEFAULT_STATUS_CODE);
            return status_result(status_parsed, resp);
        }
        _ => (),
    }

    if let Some(ep) = req.get_query_parameter(STATUS_QUERY_PARAM) {
        let status_parsed = ep.parse::<u16>().unwrap_or(DEFAULT_STATUS_CODE);
        return status_result(status_parsed, resp);
    }

    let req_url = req.get_url();
    let path_segments: Vec<&str> = req_url
        .path_segments()
        .ok_or("cannot be base")
        .unwrap()
        .collect();

    // If the path segment is too short, then just return a 500
    if path_segments.len() < 2 {
        resp.set_status(ERROR_STATUS_CODE);
        let data = serde_json::json!({ "error": "unable to parse status code properly. Try sending request like /status/302"});
        let _ = resp.set_body_json(&data);
        return Ok(resp);
    }

    let status_str = path_segments[1];
    let status_parsed = status_str.parse::<u16>().unwrap_or(DEFAULT_STATUS_CODE);

    status_result(status_parsed, resp)
}

/// Helper function to set the status code on a response.
///
/// # Arguments
///
/// * `status_u16` - The HTTP status code to set (e.g., 200, 404, 500)
/// * `resp` - The response object to modify
///
/// # Returns
///
/// Returns the modified response with the status code set.
fn status_result(status_u16: u16, mut resp: Response) -> Result<Response, Error> {

    resp.set_status(status_u16);
    Ok(resp)

    // return match status_u16 {
    //     status_int => {
    //         // https://docs.rs/fastly/latest/fastly/http/struct.StatusCode.html
    //         resp.set_status(status_int);
    //         Ok(resp)
    //     }
    //     _ => {
    //         resp.set_status(500);
    //         let data = serde_json::json!({ "error": "unable to parse status code properly. Try sending request like /status/302"});
    //         let _ = resp.set_body_json(&data);
    //         Ok(resp)
    //     }
    // };
}

/// Serves the Swagger UI HTML page from the KV store.
///
/// This is the default handler for the root path `/` and displays
/// the OpenAPI documentation interface.
///
/// # Arguments
///
/// * `resp` - The response object to populate
///
/// # Returns
///
/// Returns the Swagger UI HTML page with appropriate content type.
///
/// # Errors
///
/// Returns an error if the KV store lookup fails.
fn swagger_ui_html(mut resp: Response) -> Result<Response, Error> {
    // Define a KV store instance using the resource link name
    let store: KVStore = KVStore::open(KV_STORE_NAME)?.unwrap();

    // Get the value back from the KV store (as a string)
    let swagger_html: Body = store.lookup(SWAGGER_HTML_KEY)?.take_body();

    resp.set_body(swagger_html);
    Ok(resp)
}

/// Retrieves and serves static assets from the KV store.
///
/// Automatically sets the correct `Content-Type` header based on file extension.
/// Supports: `.js`, `.css`, `.html`, `.json`, `.jpg`, `.png`
///
/// # Arguments
///
/// * `req` - The incoming HTTP request
/// * `resp` - The response object to populate
///
/// # Returns
///
/// Returns the requested static asset with appropriate content type.
///
/// # Errors
///
/// Returns an error if the KV store lookup fails or the asset doesn't exist.
fn get_static_asset(req: &Request, mut resp: Response) -> Result<Response, Error> {
    let req_url = req.get_url();
    let path_segments: Vec<&str> = req_url
        .path_segments()
        .ok_or("cannot be base")
        .unwrap()
        .collect();

    let req_filename = path_segments.last().cloned().unwrap_or("Not Found");

    // Define a KV store instance using the resource link name
    let store = KVStore::open(KV_STORE_NAME)?.unwrap();

    // Get the value back from the KV store
    let static_asset: Body = store.lookup(req_filename)?.take_body();

    let static_filename_parts = req_filename.split('.').collect::<Vec<&str>>();
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

    Ok(resp)
}

/// Echoes back custom headers prefixed with `x-` as response headers.
///
/// For each request header starting with `x-`, this function creates a
/// corresponding response header with the prefix `resp-`.
///
/// # Example
///
/// Request header: `x-custom: value`
/// Response header: `resp-x-custom: value`
///
/// # Arguments
///
/// * `req` - The incoming HTTP request
/// * `resp` - The response object to populate
///
/// # Returns
///
/// Returns a response containing echoed custom headers.
fn set_headers(req: Request, mut resp: Response) -> Result<Response, Error> {
    for (name, value) in req.get_headers() {
        if name.as_str().starts_with(CUSTOM_HEADER_PREFIX) {
            resp.set_header(format!("{}{}", RESPONSE_HEADER_PREFIX, name.as_str()), value);
        }
    }
    Ok(resp)
}

/// Returns geolocation data for a client IP address.
///
/// By default, uses the actual client IP address. Optionally accepts an `ip`
/// query parameter to look up data for a different IP address.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request
/// * `resp` - The response object to populate
///
/// # Returns
///
/// Returns a JSON response with geolocation data including:
/// - IP address
/// - AS (Autonomous System) information
/// - Geographic location (city, country, continent)
/// - Connection information (speed, type)
/// - Proxy information (if applicable)
///
/// # Errors
///
/// Returns an error if IP parsing or geo lookup fails.
fn client_ip_data(req: Request, mut resp: Response) -> Result<Response, Error> {
    // Attempt to get the 'ip' query parameter.
    // If present, try to parse it as an IpAddr; if that fails, or if not present,
    // use the client's actual IP address.
    let ip_addr: std::net::IpAddr = if let Some(ip_param) = req.get_query_parameter(IP_QUERY_PARAM) {
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
