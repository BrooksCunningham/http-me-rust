use fastly::http::StatusCode;
use fastly::Request;
use fastly::Response;

/// Returns a GRIP response to initialize a stream
///
/// When Compute receives a non-WebSocket request (i.e. normal HTTP) and wants
/// to make it long lived (longpoll or SSE), we call handoff_fanout on it, and
/// Fanout will then forward that request to the nominated backend.  In this app,
/// that backend is this same Compute service, where we then need to respond
/// with some Grip headers to tell Fanout to hold the connection for streaming.
/// This function constructs such a response.
pub fn grip_response(ctype: &str, ghold: &str, chan: &str) -> Response {
    Response::from_status(StatusCode::OK)
        .with_header("Content-Type", ctype)
        .with_header("Grip-Hold", ghold)
        .with_header("Grip-Channel", chan)
        .with_body("{\"msg\":\"hello world\"}")
}

/// Returns a WebSocket-over-HTTP formatted TEXT message
pub fn ws_text(msg: &str) -> Vec<u8> {
    format!("TEXT {:02x}\r\n{}\r\n", msg.len(), msg)
        .as_bytes()
        .to_vec()
}

// Returns a channel-subscription command in a WebSocket-over-HTTP format
pub fn ws_sub(ch: &str) -> Vec<u8> {
    ws_text(format!("c:{{\"type\":\"subscribe\",\"channel\":\"{}\"}}", ch).as_str())
}

pub fn handle_fanout_ws(mut req: Request, chan: &str) -> Response {
    if req.get_header_str("Content-Type") != Some("application/websocket-events") {
        return Response::from_status(StatusCode::BAD_REQUEST)
            .with_body("Not a WebSocket-over-HTTP request.\n");
    }

    let req_body: Vec<u8> = req.take_body().into_bytes();
    println!("req_body for ws");
    // println!("{:?}", std::str::from_utf8(&req_body));
    let mut resp_body: Vec<u8> = [].to_vec();

    let mut resp = Response::from_status(StatusCode::OK)
        .with_header("Content-Type", "application/websocket-events");

    if req_body.starts_with(b"OPEN\r\n") {
        resp.set_header("Sec-WebSocket-Extensions", "grip; message-prefix=\"\"");
        resp_body.extend("OPEN\r\n".as_bytes());
        resp_body.extend(ws_sub(chan));
    } else if req_body.starts_with(b"TEXT ") {
        resp_body.extend(ws_text(
            format!("You said: {}", std::str::from_utf8(&req_body).unwrap_or("")).as_str(),
        ));
    }
    println!("ws resp_body");
    println!("{:?}", std::str::from_utf8(&resp_body));

    resp.set_body(resp_body);
    return resp;
}
