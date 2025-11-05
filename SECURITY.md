# Security Policy

## Reporting Security Issues

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in HTTP-ME, please report it responsibly:

### Preferred Method: Private Security Advisory

1. Go to the [Security Advisories](https://github.com/BrooksCunningham/http-me-rust/security/advisories) page
2. Click "Report a vulnerability"
3. Provide detailed information about the vulnerability

### Alternative Contact Methods

- **Email**: brookscunningham@gmail.com
- **LinkedIn**: Contact Brooks Cunningham via [LinkedIn](https://www.linkedin.com/in/brooks-cunningham/)

## What to Include in Your Report

Please include as much of the following information as possible:

- **Type of vulnerability** (e.g., XSS, CSRF, injection, etc.)
- **Affected endpoints or components**
- **Step-by-step instructions** to reproduce the issue
- **Proof of concept** or exploit code (if available)
- **Potential impact** of the vulnerability
- **Suggested fix** (if you have one)

## Response Timeline

- **Initial Response**: Within 48 hours of receiving your report
- **Status Update**: Within 7 days with an assessment and timeline
- **Resolution**: Depends on severity and complexity
  - Critical: Within 7 days
  - High: Within 30 days
  - Medium/Low: Within 90 days

## Security Update Process

When a security vulnerability is confirmed:

1. **Assessment**: We evaluate the severity and impact
2. **Fix Development**: We develop and test a fix
3. **Notification**: We notify affected users if applicable
4. **Release**: We release a patched version
5. **Disclosure**: We publish a security advisory (coordinated with reporter)

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| Older   | :x:                |

We only support the latest version deployed to production. Since this is a Compute@Edge service, updates are automatically deployed.

## Security Considerations

### Current Security Features

- ✅ **HTTPS Only**: All traffic is encrypted in transit
- ✅ **DDoS Protection**: Built-in Fastly DDoS mitigation
- ✅ **WAF Integration**: Next-Gen WAF (NGWAF) testing in CI/CD
- ✅ **Edge Security**: Runs on Fastly's secure edge platform
- ✅ **CORS Headers**: Properly configured CORS for cross-origin requests
- ✅ **Input Validation**: Status codes and parameters are validated

### Known Limitations

This is an intentionally open HTTP testing service designed to:
- Echo back request data (by design)
- Accept arbitrary status codes (by design)
- Allow custom headers (by design)

These behaviors are features, not bugs, for testing purposes.

### Out of Scope

The following are **not** considered security vulnerabilities for this service:

- **Information Disclosure**: Echoing request data is the intended purpose
- **Open Redirects**: Via status code endpoints (3xx responses are intentional)
- **DoS via Tarpit**: The tarpit feature is intentional for testing
- **Arbitrary Status Codes**: Allowing any status code is by design
- **Missing Rate Limiting**: Handled at the Fastly platform level

## Best Practices for Users

If you're using HTTP-ME in your applications:

- ⚠️ **Never send sensitive data**: This is a public testing service
- ⚠️ **Don't use in production**: Use for testing and development only
- ⚠️ **Be aware of data logging**: Request data may be logged
- ⚠️ **Use HTTPS**: Always use the HTTPS endpoint

## Vulnerability Disclosure Policy

We follow responsible disclosure practices:

- We request **90 days** before public disclosure
- We will work with you to understand and fix the issue
- We will credit you in the security advisory (if desired)
- We appreciate coordinated disclosure

## Security Acknowledgments

We thank the security researchers and contributors who help keep HTTP-ME secure:

*(This section will be updated as vulnerabilities are reported and fixed)*

## Questions?

If you have questions about this security policy, please contact:
- **Email**: brookscunningham@gmail.com
- **GitHub Issues**: For general questions (not vulnerabilities)

---

**Last Updated**: November 2024
