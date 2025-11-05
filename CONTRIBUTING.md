# Contributing to HTTP-ME

Thank you for your interest in contributing to HTTP-ME! This document provides guidelines and information to help you contribute effectively.

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (version 1.83.0 or later)
- [Fastly CLI](https://developer.fastly.com/learning/tools/cli/)
- Git

### Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/BrooksCunningham/http-me-rust.git
   cd http-me-rust
   ```

2. **Install Rust and add WASM target:**
   ```bash
   rustup target add wasm32-wasip1
   ```

3. **Install Fastly CLI:**
   Follow the [Fastly CLI installation guide](https://developer.fastly.com/learning/tools/cli/#installing)

## Building the Project

Build the project using Cargo:

```bash
cargo build
```

For Fastly Compute@Edge builds:

```bash
fastly compute build
```

## Running Locally

To run the service locally using Fastly's local development server:

```bash
fastly compute serve
```

The service will be available at `http://localhost:7676`

### Testing Locally

Once the local server is running, you can test endpoints:

```bash
# Test status endpoint
curl -i http://localhost:7676/status/404

# Test anything endpoint
curl http://localhost:7676/anything/test?foo=bar

# Test with custom headers
curl -H 'endpoint:status=302' http://localhost:7676/test
```

## Code Quality

### Linting

We use Clippy for linting. Run it before submitting changes:

```bash
cargo clippy -- -W clippy::all
```

All clippy warnings should be addressed.

### Formatting

Use `rustfmt` to format your code:

```bash
cargo fmt
```

### Code Standards

- **Documentation**: Add doc comments (`///`) for all public functions and modules
- **Constants**: Use named constants instead of magic strings or numbers
- **Error Handling**: Properly handle and propagate errors using `Result<T, Error>`
- **CORS**: Ensure endpoints that need CORS support include appropriate headers
- **Performance**: Be mindful of allocations and cloning in hot paths

## Project Structure

```
http-me-rust/
├── src/
│   └── main.rs          # Main application code with all endpoint handlers
├── static-assets/       # Static files served via KV store
│   ├── swagger.html     # Swagger UI
│   ├── openapi-spec.json # OpenAPI specification
│   └── ...             # Other static assets
├── fastly.toml          # Fastly service configuration
├── Cargo.toml           # Rust dependencies
└── README.md            # Project documentation
```

## Adding New Endpoints

When adding a new endpoint:

1. **Define constants** at the top of `main.rs` for any new paths, headers, or configuration values
2. **Create a handler function** with proper documentation:
   ```rust
   /// Brief description of what the endpoint does.
   ///
   /// # Arguments
   ///
   /// * `req` - The incoming HTTP request
   /// * `resp` - The response object to populate
   ///
   /// # Returns
   ///
   /// Description of what is returned
   ///
   /// # Errors
   ///
   /// Description of possible errors
   fn my_endpoint(req: Request, mut resp: Response) -> Result<Response, Error> {
       // Implementation
   }
   ```
3. **Add routing** in the `handler()` function
4. **Update documentation** including README.md and OpenAPI spec if applicable
5. **Test the endpoint** locally before submitting

## Static Assets

Static assets are stored in the `static-assets/` directory and served via Fastly's KV Store.

To add new static assets:

1. Add the file to `static-assets/`
2. Update `fastly.toml` to include the new file in the `local_server.kv_stores` section
3. For production, the GitHub Actions workflow will update the KV store automatically

## OpenAPI Specification

The API is documented using OpenAPI 3.0. The specification is located at `static-assets/openapi-spec.json`.

When adding or modifying endpoints, update the OpenAPI spec accordingly.

## Testing

Currently, the project focuses on manual testing using the local Fastly server. Future contributions to add automated testing are welcome.

### Manual Testing Checklist

When making changes, test:

- [ ] All affected endpoints work correctly
- [ ] Error cases are handled properly
- [ ] CORS headers are present where needed
- [ ] Status codes are appropriate
- [ ] Response formats match the OpenAPI spec

## Pull Request Process

1. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code standards above

3. **Test your changes** locally using `fastly compute serve`

4. **Commit your changes** with clear, descriptive commit messages:
   ```bash
   git commit -m "Add feature: description of what you added"
   ```

5. **Push your branch**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** on GitHub with:
   - A clear title describing the change
   - A description of what changed and why
   - Any testing you performed
   - References to related issues

## CI/CD

The project uses GitHub Actions for continuous integration and deployment:

- **Test & Build**: On every push, the code is built and tested
- **Deploy**: On pushes to `main`, the service is deployed to Fastly Compute@Edge
- **WAF Testing**: Tests include Next-Gen WAF (NGWAF) integration tests

## Common Tasks

### Updating Dependencies

```bash
cargo update
```

Check for outdated dependencies:

```bash
cargo outdated
```

### Adding a New Dependency

1. Add to `Cargo.toml`:
   ```toml
   [dependencies]
   new-crate = "1.0"
   ```

2. Run `cargo build` to fetch and build the dependency

## Getting Help

- **Issues**: Open an issue on GitHub for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Security**: See [SECURITY.md](SECURITY.md) for reporting security issues

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment for all contributors

## License

By contributing to HTTP-ME, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to HTTP-ME! 🚀
