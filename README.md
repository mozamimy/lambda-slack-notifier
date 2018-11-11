# lambda-slack-notifier

[![CircleCI](https://circleci.com/gh/mozamimy/lambda-slack-notifier/tree/master.svg?style=svg)](https://circleci.com/gh/mozamimy/lambda-slack-notifier/tree/master)

## About

This program sends a message to Slack when it receives like following event data published from AWS SNS topic.

Option elements can be omitted from `MessageAttributes` object. In that case, this function fills omitted these options with default values of incoming-webhook.

```json
{
  "Records": [
    {
      "EventSource": "aws:sns",
      "EventVersion": "1.0",
      "EventSubscriptionArn": "arn:aws:sns:us-east-1::ExampleTopic",
      "Sns": {
        "Type": "Notification",
        "MessageId": "95df01b4-ee98-5cb9-9903-4c221d41eb5e",
        "TopicArn": "arn:aws:sns:us-east-1:123456789012:ExampleTopic",
        "Subject": "example subject",
        "Message": "example message",
        "Timestamp": "1970-01-01T00:00:00.000Z",
        "SignatureVersion": "1",
        "Signature": "EXAMPLE",
        "SigningCertUrl": "EXAMPLE",
        "UnsubscribeUrl": "EXAMPLE",
        "MessageAttributes": {
          "Channel": {
            "Type": "String",
            "Value": "#nanika-channel"
          },
          "UserName": {
            "Type": "String",
            "Value": "nanika"
          },
          "IconEmoji": {
            "Type": "String",
            "Value": ":rabbit:"
          },
          "Color": {
            "Type": "String",
            "Value": "warning"
          }
        }
      }
    }
  ]
}
```

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
