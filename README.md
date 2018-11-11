# lambda-slack-notifier

[![CircleCI](https://circleci.com/gh/mozamimy/lambda-slack-notifier/tree/master.svg?style=svg)](https://circleci.com/gh/mozamimy/lambda-slack-notifier/tree/master)

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
