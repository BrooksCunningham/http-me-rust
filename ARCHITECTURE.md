# Architecture Overview

## System Architecture

HTTP-ME is a serverless HTTP testing service built on Fastly Compute@Edge, compiled from Rust to WebAssembly (WASM).

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │ HTTPS Request
       ▼
┌─────────────────────────────────┐
│  Fastly Edge Network (Global)   │
│  ┌───────────────────────────┐  │
│  │  Compute@Edge (WASM)      │  │
│  │  ┌─────────────────────┐  │  │
│  │  │   HTTP-ME Service   │  │  │
│  │  │   (Rust → WASM)     │  │  │
│  │  └─────────────────────┘  │  │
│  └───────────────────────────┘  │
│              │                  │
│              ▼                  │
│  ┌───────────────────────────┐  │
│  │   KV Store (Static Assets)│  │
│  └───────────────────────────┘  │
└─────────────────────────────────┘
       │
       ▼
┌──────────────┐
│   Response   │
└──────────────┘
```

## Core Components

### 1. Request Handler (`main()`)

**Purpose**: Entry point for all HTTP requests

**Responsibilities**:
- Receive incoming requests
- Apply compression hints
- Handle tarpit mode for slow responses
- Send responses to clients

**Flow**:
```rust
main() → handler() → endpoint_function() → Response
```

### 2. Router (`handler()`)

**Purpose**: Route requests to appropriate endpoint handlers

**Routing Strategy**:
1. Check custom headers (e.g., `endpoint:status=404`)
2. Check query parameters (e.g., `?status=404`)
3. Match URL path prefix
4. Return default response

**Routes**:
- `/` → `swagger_ui_html()`
- `/status/{code}` → `status()`
- `/anything/{path}` → `anything()`
- `/static-assets/*` → `get_static_asset()`
- `/forms/post` → `get_static_asset()`
- `/utilities/set_headers` → `set_headers()`
- `/client_ip_data` → `client_ip_data()`

### 3. Endpoint Handlers

Each endpoint handler follows a consistent pattern:

```rust
fn endpoint_name(req: Request, mut resp: Response) -> Result<Response, Error> {
    // 1. Parse request data
    // 2. Process business logic
    // 3. Build response
    // 4. Return Result
}
```

#### Key Endpoints:

**`status()`**
- Returns specified HTTP status code
- Supports multiple input methods (header, query, path)
- Validates status codes (returns 500 if invalid)

**`anything()`**
- Echoes all request data as JSON
- Handles CORS preflight (OPTIONS)
- Returns headers, body, query params, IP, method, URL

**`client_ip_data()`**
- Performs IP geolocation lookup
- Accepts IP via query parameter or uses client IP
- Returns comprehensive geo data (AS, location, connection info)

**`get_static_asset()`**
- Serves files from KV Store
- Automatically sets Content-Type based on extension
- Supports: JS, CSS, HTML, JSON, images

**`set_headers()`**
- Echoes custom request headers (prefixed with `x-`)
- Adds `resp-` prefix to response headers

## Data Flow

### Normal Request Flow

```
1. Client Request
   ↓
2. Fastly Edge (TLS termination, DDoS protection)
   ↓
3. main() - Entry point
   ↓
4. handler() - Routing logic
   ↓
5. Endpoint Handler - Business logic
   ↓
6. Response Construction
   ↓
7. main() - Send to client
   ↓
8. Client Response
```

### Tarpit Mode Flow

```
1. Client Request with "endpoint:tarpit=1" header
   ↓
2. handler() sets "action-tarpit" header
   ↓
3. main() detects tarpit header
   ↓
4. Stream response in chunks:
   - 100 bytes every 1 second
   ↓
5. Complete streaming
```

## Storage Architecture

### KV Store

Fastly KV Store is used for static assets:

```
KV Store: "assets_store"
├── swagger.html
├── openapi-spec.json
├── *.js (JavaScript files)
├── *.css (Stylesheets)
├── *.html (HTML pages)
└── *.jpg, *.png (Images)
```

**Access Pattern**:
1. Request comes for `/static-assets/{filename}`
2. Extract filename from path
3. Lookup in KV store: `KVStore::open("assets_store")`
4. Return asset with correct Content-Type

**Local Development**: Configured in `fastly.toml`
**Production**: Updated via GitHub Actions after deployment

## Constants and Configuration

All magic strings and values are defined as constants at the top of `main.rs`:

```rust
// KV Store
const KV_STORE_NAME: &str = "assets_store";
const SWAGGER_HTML_KEY: &str = "swagger.html";

// Headers
const COMPRESS_HINT_HEADER: &str = "x-compress-hint";
const TARPIT_ACTION_HEADER: &str = "action-tarpit";
const ENDPOINT_HEADER: &str = "endpoint";

// Query Parameters
const STATUS_QUERY_PARAM: &str = "status";
const IP_QUERY_PARAM: &str = "ip";

// Paths
const PATH_STATUS: &str = "/status";
const PATH_ANYTHING: &str = "/anything";
// ... etc
```

**Benefits**:
- Easy to modify configuration
- Better IDE/Copilot autocomplete
- Reduces typos and bugs
- Improves code maintainability

## Error Handling

### Error Strategy

1. **Propagate with `?`**: For recoverable errors
2. **`unwrap()` with context**: For errors that "shouldn't happen"
3. **`unwrap_or(default)`**: For optional values with sensible defaults
4. **Custom error responses**: Return 500 with JSON error message

### Example:

```rust
// Propagate KV Store errors
let store = KVStore::open(KV_STORE_NAME)?.unwrap();

// Default to 200 if parsing fails
let status_parsed = status_str.parse::<u16>().unwrap_or(DEFAULT_STATUS_CODE);

// Custom error for invalid path
if path_segments.len() < 2 {
    resp.set_status(ERROR_STATUS_CODE);
    let data = json!({ "error": "unable to parse status code" });
    resp.set_body_json(&data)?;
    return Ok(resp);
}
```

## Performance Considerations

### Edge Execution
- **Cold Start**: ~1-5ms (WASM is fast)
- **Warm Execution**: <1ms
- **Geographic Distribution**: Runs in 100+ edge locations globally

### Optimizations
- **Zero-copy where possible**: Use references instead of cloning
- **Minimal allocations**: Reuse response objects
- **Streaming for tarpit**: Avoid buffering large responses
- **Compression hints**: Enable dynamic compression at edge

### Limitations
- **No async I/O**: Fastly Compute@Edge is synchronous
- **No threads**: Single-threaded execution
- **Memory limit**: 128MB per instance
- **Execution time**: 60s timeout (configurable)

## CORS Implementation

CORS headers are set for cross-origin requests:

```rust
const CORS_ALLOW_ORIGIN: &str = "*";
const CORS_ALLOW_METHODS: &str = "GET, POST, PUT, DELETE, OPTIONS";
const CORS_ALLOW_HEADERS: &str = "Content-Type, Authorization";
```

**Preflight Handling** (OPTIONS requests):
```rust
if req.get_method() == Method::OPTIONS {
    return Ok(Response::from_status(StatusCode::OK)
        .with_header("Access-Control-Allow-Origin", CORS_ALLOW_ORIGIN)
        .with_header("Access-Control-Allow-Methods", CORS_ALLOW_METHODS)
        .with_header("Access-Control-Allow-Headers", CORS_ALLOW_HEADERS)
        .with_body(Body::new()));
}
```

## Security Model

### Trust Boundaries

```
┌──────────────────────────────────┐
│  External (Untrusted)            │
│  - Client requests               │
│  - Headers, query params, body   │
└──────────────┬───────────────────┘
               │ Validation
               ▼
┌──────────────────────────────────┐
│  Application (Trusted)           │
│  - Parsed and validated input    │
│  - Business logic                │
└──────────────┬───────────────────┘
               │
               ▼
┌──────────────────────────────────┐
│  Fastly Platform (Trusted)       │
│  - KV Store                      │
│  - Geo lookup                    │
│  - Edge infrastructure           │
└──────────────────────────────────┘
```

### Input Validation

- **Status codes**: Parsed with `parse::<u16>()`, defaults to 200 on error
- **IP addresses**: Validated with `parse::<IpAddr>()`, falls back to client IP
- **Filenames**: Sanitized through path segment extraction
- **Headers**: Validated by Fastly platform before reaching code

### Security Features

1. **DDoS Protection**: Fastly's built-in protection
2. **WAF Integration**: Next-Gen WAF testing in CI/CD
3. **HTTPS Only**: All connections encrypted
4. **No SQL Injection**: No database queries
5. **No XSS**: No user content rendering (JSON responses)

## CI/CD Pipeline

### GitHub Actions Workflow

```
┌─────────────┐
│  Push to    │
│  main       │
└──────┬──────┘
       │
       ▼
┌─────────────────────┐
│  Test & Build Job   │
│  1. Setup Rust      │
│  2. Build WASM      │
│  3. Run local serve │
│  4. Install NGWAF   │
│  5. Test endpoints  │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│  Deploy Job         │
│  1. Build package   │
│  2. Deploy to edge  │
│  3. Update KV Store │
└─────────────────────┘
```

### Deployment Strategy

- **Zero Downtime**: New version activates instantly
- **Rollback**: Previous versions available for instant rollback
- **Global Distribution**: Deploys to all edge locations simultaneously

## Monitoring and Observability

### Logging

```rust
println!("FASTLY_SERVICE_VERSION: {}", version);
println!("{}", query_param); // Debug logging
```

**Logs available**:
- Fastly real-time logs
- Aggregated to logging endpoint (if configured)
- GitHub Actions build logs

### Metrics

Automatically collected by Fastly:
- Request count
- Response time (P50, P95, P99)
- Error rates
- Bandwidth
- Cache hit ratio (for static assets)

## Extension Points

### Adding New Endpoints

1. **Define constants** for paths/headers
2. **Create handler function** with documentation
3. **Add route** in `handler()` function
4. **Update OpenAPI spec**
5. **Add tests**

### Adding New Features

1. **Review architecture** for fit
2. **Update constants** if needed
3. **Implement feature** with docs
4. **Test locally** with `fastly compute serve`
5. **Submit PR** with tests

## Future Enhancements

Potential improvements:
- [ ] Request/response validation against schemas
- [ ] More sophisticated routing (regex patterns)
- [ ] Request recording and replay
- [ ] Webhook testing endpoints
- [ ] WebSocket support (when available in Compute@Edge)
- [ ] Automated testing framework
- [ ] Performance benchmarking suite

---

**Last Updated**: November 2024
