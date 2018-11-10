extern crate aws_lambda as lambda;
#[macro_use] extern crate log;

use lambda::event::s3::S3Event;

fn main() {
    lambda::start(|input: S3Event| {
        let mut names = Vec::new();
        for record in input.records {
            names.push(record.event_name)
        }
        Ok(format!("Event names:\n{:#?}", names))
    })
}
