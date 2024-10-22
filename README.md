[![Test, build, and deploy](https://github.com/BrooksCunningham/http-me-rust/actions/workflows/test_build_deploy.yaml/badge.svg)](https://github.com/BrooksCunningham/http-me-rust/actions/workflows/test_build_deploy.yaml)

# Website for HTTP testing 

[https://http-me.edgecompute.app/](https://http-me.edgecompute.app/)

# Demo for DevSecOps 

Using rust to demo DevSecOps. 

Check out the Github Actions. The Github Actions are defined in the folder `.github/workflows/`

# Sending test requests

Use curl to send requests to https://http-me.edgecompute.app

# customize the landing page with Swagger UI using Fastly KV Store

## creates the kv store
`fastly kv-store create --name=assets_store`

## get the kv store id that was just created
`fastly kv-store list | grep assets_store -a1 | grep ID`

## add everything in the directory for static assets in the store-id
`fastly kv-store-entry create --dir ./static-assets/ --store-id=STORE-ID`

## create a link between the kv store and the service
`fastly resource-link create --version=latest --autoclone --resource-id=STORE-ID`

## activate the latest version
`fastly service-version activate --version=latest`

https://developer.fastly.com/reference/cli/kv-store/
https://developer.fastly.com/reference/cli/kv-store-entry/


# How to send requests through the CLI

## Return a specific status code
`curl -i 'https://http-me.edgecompute.app/status/302'`

## Return data sent to the server
`curl -i 'https://http-me.edgecompute.app/anything/myquery?foo=bar'`

## return a custom status code at an arbitrary path

`curl -i 'https://http-me.edgecompute.app/any/path/myquery?foo=bar' -H 'endpoint:status=302'`

# WORK IN PROGRESS - Not Yet working - Testing with websockets

```
curl https://http-me.edgecompute.app/chatroom/test/websocket
curl https://http-me.edgecompute.app/chatroom/test/stream
curl https://http-me.edgecompute.app/chatroom/test/sse
curl https://http-me.edgecompute.app/chatroom/test/long-poll
```

For requests made to domains ending in `.edgecompute.app`, the app will handle requests to the following endpoints without forwarding to the origin:

* `/chatroom/test/websocket`: bi-directional WebSocket
* `/chatroom/test/stream`: HTTP streaming of `text/plain`
* `/chatroom/test/sse`: SSE (streaming of `text/event-stream`)
* `/chatroom/test/long-poll`: Long-polling

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.








