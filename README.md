# Demo for DevSecOps

Using rust to demo DevSecOps. 

Check out the Github Actions. The Github Actions are defined in the folder `.github/workflows/`

# Sending test requests

Here are some example curl requests.
## return a specific status code
`curl -i 'https://http-me.edgecompute.app/status/302'`

## Return data sent to the server
`curl -i 'https://http-me.edgecompute.app/anything/myquery?foo=bar'`

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.
