# replace-mock-backend

Establish connection to receive datas:

```shell
curl -v http://127.0.0.1:7999/stream
```

Publish text to receivers:

```shell
curl -v http://0.0.0.0:5561/publish \
 -d '{ "items": [{ "channel": "test", "formats": { "http-stream": { "content": "hello there\n" } } }] }'
```
