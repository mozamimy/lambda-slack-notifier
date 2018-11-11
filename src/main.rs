extern crate aws_lambda as lambda;
extern crate reqwest;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use lambda::event::sns::SnsEvent;

#[derive(Debug, Serialize, Deserialize)]
struct Field {
    title: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Attachment {
    color: Option<String>,
    fallback: String,
    fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    channel: String,
    username: String,
    icon_emoji: String,
    attachments: Vec<Attachment>,
}

fn main() {
    lambda::start(|input: SnsEvent| {
        let slack_webhook = std::env::var("SLACK_WEBHOOK")?;

        for record in input.records {
            let subject = record.sns.subject.unwrap_or(String::from(""));
            let message = record.sns.message.unwrap_or(String::from(""));

            println!("subject: {:?}", subject);
            println!("message: {:?}", message);
            println!("message_attributes: {:?}", record.sns.message_attributes);

            let mut payload = Payload {
                channel: String::new(),
                username: String::new(),
                icon_emoji: String::new(),
                attachments: Vec::<Attachment>::new(),
            };
            payload.attachments.push(Attachment {
                color: None,
                fallback: message.clone(),
                fields: Vec::<Field>::new(),
            });
            payload.attachments.last_mut().unwrap().fields.push(Field {
                title: subject,
                value: message,
            });
            for attribute in &record.sns.message_attributes {
                match attribute.0.as_str() {
                    "Channel" => {
                        attribute
                            .1
                            .get("Value")
                            .map(|v| v.as_str().map(|s| payload.channel = s.to_string()));
                    }
                    "UserName" => {
                        attribute
                            .1
                            .get("Value")
                            .map(|v| v.as_str().map(|s| payload.username = s.to_string()));
                    }
                    "IconEmoji" => {
                        attribute
                            .1
                            .get("Value")
                            .map(|v| v.as_str().map(|s| payload.icon_emoji = s.to_string()));
                    }
                    "Color" => {
                        attribute.1.get("Value").map(|v| {
                            v.as_str().map(|s| {
                                for attachment in payload.attachments.iter_mut() {
                                    attachment.color = Some(s.to_string());
                                }
                            });
                        });
                    }
                    _ => println!("Warning: Unknown message attribute: {:?}", attribute.1),
                }
            }

            println!("Request JSON: {}", serde_json::to_string(&payload).unwrap());

            let client = reqwest::Client::builder().redirect(reqwest::RedirectPolicy::none()).build()?;
            let resp = client.post(&slack_webhook).json(&payload).send()?;

            if !resp.status().is_success() {
                return Err(format_err!("Slack API returns non success HTTP code: {}", resp.status()));
            }
        }
        Ok("ok")
    })
}
