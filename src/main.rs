extern crate aws_lambda as lambda;
extern crate reqwest;
#[macro_use]
extern crate failure;

use lambda::event::sns::SnsEvent;

fn main() {
    lambda::start(|input: SnsEvent| {
        let slack_webhook = std::env::var("SLACK_WEBHOOK")?;

        for record in input.records {
            let subject = record.sns.subject.unwrap_or(String::from(""));
            let message = record.sns.message.unwrap_or(String::from(""));

            println!("subject: {:?}", subject);
            println!("message: {:?}", message);

            let mut params = std::collections::HashMap::new();
            params.insert("text", message);
            let client = reqwest::Client::builder().redirect(reqwest::RedirectPolicy::none()).build()?;
            let resp = client.post(&slack_webhook).json(&params).send()?;

            if !resp.status().is_success() {
                return Err(format_err!("Slack API returns non success HTTP code: {}", resp.status()));
            }
        }
        Ok("ok")
    })
}
