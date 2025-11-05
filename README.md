[![Test, build, and deploy](https://github.com/BrooksCunningham/http-me-rust/actions/workflows/test_build_deploy.yaml/badge.svg)](https://github.com/BrooksCunningham/http-me-rust/actions/workflows/test_build_deploy.yaml)

# HTTP-ME: HTTP Testing Service

A lightweight HTTP testing and debugging service built on Fastly Compute@Edge. Inspired by httpbin, HTTP-ME provides endpoints for testing HTTP clients, debugging requests, and simulating various HTTP scenarios.

🌐 **Live Service**: [https://http-me.edgecompute.app/](https://http-me.edgecompute.app/)

## Features

- ✅ **Status Code Testing**: Return any HTTP status code
- 🔍 **Request Inspection**: Echo back request headers, body, and metadata
- 🌍 **IP Geolocation**: Get geolocation data for any IP address
- 🎨 **Static Assets**: Serve static files and interactive Swagger UI
- 🐌 **Tarpit Mode**: Simulate slow responses for timeout testing
- 🔧 **Header Utilities**: Test custom header handling
- 📡 **CORS Support**: Built-in CORS support for cross-origin requests

## Quick Start

### Testing Status Codes

Return a specific HTTP status code:

```bash
# Get a 404 Not Found
curl -i https://http-me.edgecompute.app/status/404

# Get a 302 Redirect
curl -i https://http-me.edgecompute.app/status/302

# Get a 500 Internal Server Error
curl -i https://http-me.edgecompute.app/status/500
```

### Inspecting Requests

Echo back all request data (headers, body, query params):

```bash
# Basic request inspection
curl https://http-me.edgecompute.app/anything/test

# With query parameters
curl https://http-me.edgecompute.app/anything/mypath?foo=bar&baz=qux

# With request body (POST)
curl -X POST https://http-me.edgecompute.app/anything/test \
  -H "Content-Type: application/json" \
  -d '{"key":"value"}'
```

### Custom Status via Headers

Set a custom status code using headers at any path:

```bash
curl -i https://http-me.edgecompute.app/any/path \
  -H 'endpoint:status=302'
```

### IP Geolocation

Get geolocation data for your IP or a specific IP:

```bash
# Get data for your own IP
curl https://http-me.edgecompute.app/client_ip_data

# Get data for a specific IP
curl https://http-me.edgecompute.app/client_ip_data?ip=8.8.8.8
```

### Testing Custom Headers

Echo back custom headers prefixed with `x-`:

```bash
curl https://http-me.edgecompute.app/utilities/set_headers \
  -H "x-custom-header: my-value" \
  -H "x-another: test"
```

## API Endpoints

| Endpoint | Description | Example |
|----------|-------------|---------|
| `/` | Swagger UI documentation | `curl https://http-me.edgecompute.app/` |
| `/status/{code}` | Return specified HTTP status code | `curl -i https://http-me.edgecompute.app/status/404` |
| `/anything/{path}` | Echo request data as JSON | `curl https://http-me.edgecompute.app/anything/test` |
| `/client_ip_data` | Get IP geolocation data | `curl https://http-me.edgecompute.app/client_ip_data` |
| `/utilities/set_headers` | Echo custom headers | `curl https://http-me.edgecompute.app/utilities/set_headers -H "x-test: value"` |
| `/static-assets/{file}` | Serve static files | `curl https://http-me.edgecompute.app/static-assets/openapi-spec.json` |

## Advanced Features

### Tarpit Mode

Simulate slow responses for testing timeouts:

```bash
curl -H 'endpoint:tarpit=1' https://http-me.edgecompute.app/status/200
```

This will send the response in 100-byte chunks with a 1-second delay between each chunk.

### Query Parameter Status

Alternative way to specify status codes:

```bash
curl -i 'https://http-me.edgecompute.app/any/path?status=404'
```

## Development

### Prerequisites

- [Rust](https://rustup.rs/) 1.83.0 or later
- [Fastly CLI](https://developer.fastly.com/learning/tools/cli/)

### Local Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/BrooksCunningham/http-me-rust.git
   cd http-me-rust
   ```

2. **Install dependencies:**
   ```bash
   rustup target add wasm32-wasip1
   cargo build
   ```

3. **Run locally:**
   ```bash
   fastly compute serve
   ```

4. **Test locally:**
   ```bash
   curl http://localhost:7676/status/200
   curl http://localhost:7676/anything/test
   ```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed development guidelines.

## Deployment

This service is automatically deployed to Fastly Compute@Edge via GitHub Actions when changes are pushed to the `main` branch.

### Deployment Process

1. **Test & Build**: Code is compiled and tested
2. **WAF Testing**: Tests run through Next-Gen WAF
3. **Deploy**: Service is deployed to Fastly edge network
4. **KV Store Update**: Static assets are uploaded to KV Store

See the GitHub Actions workflow in [`.github/workflows/test_build_deploy.yaml`](.github/workflows/test_build_deploy.yaml) for details.

## Architecture

Built on [Fastly Compute@Edge](https://www.fastly.com/products/edge-compute), this service runs at the edge for:
- ⚡ **Low Latency**: Responses from the nearest edge location
- 🌍 **Global Distribution**: Available worldwide
- 🔒 **Security**: Built-in DDoS protection and WAF
- 📈 **Scalability**: Automatically scales with traffic

### Technology Stack

- **Language**: Rust (compiled to WASM)
- **Platform**: Fastly Compute@Edge
- **Storage**: Fastly KV Store for static assets
- **CI/CD**: GitHub Actions

## Use Cases

- **HTTP Client Testing**: Test your HTTP client implementation
- **API Development**: Debug requests during development
- **Load Testing**: Generate predictable responses for load tests
- **Integration Testing**: Mock various HTTP scenarios
- **Network Debugging**: Inspect what your application sends
- **Education**: Learn about HTTP status codes and headers

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Contribution Guide

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Submit a pull request

## Security Issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [httpbin](https://httpbin.org/)
- Built with [Fastly Compute@Edge](https://www.fastly.com/products/edge-compute)
- Powered by [Rust](https://www.rust-lang.org/)

---

**Maintainer**: Brooks Cunningham  
**Repository**: [github.com/BrooksCunningham/http-me-rust](https://github.com/BrooksCunningham/http-me-rust)
