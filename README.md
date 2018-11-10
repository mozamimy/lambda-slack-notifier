# lambda-slack-notifier

## Run locally with SAM CLI

```sh
make build-docker-image
make build
make zip
make run
```

## Releae build

```sh
make build-docker-image
make build BUILD=release
make zip BUILD=release
```
