extern crate aws_lambda_events;
extern crate lambda_runtime;
extern crate reqwest;
extern crate serde;
extern crate simple_error;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use aws_lambda_events::event::sns::SnsEvent;
use lambda_runtime::LambdaEvent;

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

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<SnsEvent>) -> Result<(), lambda_runtime::Error> {
    let slack_webhook = std::env::var("SLACK_WEBHOOK")?;

    let (event, _context) = event.into_parts();

    for record in event.records {
        let subject = record.sns.subject.unwrap_or(String::from(""));
        let message = record.sns.message;

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
                    payload.channel = attribute.1.value.to_string();
                }
                "UserName" => {
                    payload.username = attribute.1.value.to_string();
                }
                "IconEmoji" => {
                    payload.icon_emoji = attribute.1.value.to_string();
                }
                "Color" => {
                    for attachment in payload.attachments.iter_mut() {
                        attachment.color = Some(attribute.1.value.to_string());
                    }
                }
                _ => println!("Warning: Unknown message attribute: {:?}", attribute.1),
            }
        }

        println!("Request JSON: {}", serde_json::to_string(&payload).unwrap());

        let client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build()?;
        let resp = client.post(&slack_webhook).json(&payload).send().await?;

        if !resp.status().is_success() {
            return Err(Box::new(simple_error::SimpleError::new(format!(
                "Slack API returns non success HTTP code: {}",
                resp.status()
            ))));
        }
    }
    Ok(())
}
