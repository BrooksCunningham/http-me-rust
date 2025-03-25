[![Test, build, and deploy](https://github.com/BrooksCunningham/http-me-rust/actions/workflows/test_build_deploy.yaml/badge.svg)](https://github.com/BrooksCunningham/http-me-rust/actions/workflows/test_build_deploy.yaml)

# Website for HTTP testing 
[https://http-me.edgecompute.app/](https://http-me.edgecompute.app/)

# How is this deployed?
Check out the Github Actions. The Github Actions are defined in the folder `.github/workflows/`

# Sending test requests

## Return a specific status code
`curl -i 'https://http-me.edgecompute.app/status/302'`

## Return data sent to the server
`curl 'https://http-me.edgecompute.app/anything/whydopirates?likequeryparams=args'`

## Return a custom status code at an arbitrary path
`curl -i 'https://http-me.edgecompute.app/any/path/myquery?foo=bar' -H 'endpoint:status=302'`

## Security issues
Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.

