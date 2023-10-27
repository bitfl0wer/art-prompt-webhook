use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use chrono::{Local, NaiveDate, NaiveTime};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

/// Sends a weekly message via Discord Webhook, containing a random noun.
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    webhook_url: String,
    /// The time to send the message at. Uses your system timezone. Format: HH:MM
    #[arg(short, long)]
    time: String,
    #[arg(short, long, default_value = "7")]
    interval_days: u64,
    #[arg(short, long, required = false)]
    first_execution: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct WordResponse {
    #[serde(rename = "0")]
    zero: String,
}

#[derive(Serialize, Debug)]
struct WebhookPayload {
    content: String,
    embeds: Option<String>,
    attachments: Vec<String>,
    username: String,
    avatar_url: String,
}

fn main() {
    let args = Args::parse();
    let parsed_time = NaiveTime::from_str(&args.time).expect("Unable to parse Time");
    let next_execution_date = match args.first_execution {
        Some(_) => NaiveDate::from_str(args.first_execution.as_ref().unwrap())
            .expect("Unable to parse First Execution Date. Required Format: YYYY-MM-DD"),
        None => Local::now().date_naive() + chrono::Days::new(args.interval_days),
    };
    let mut next_execution_datetime = next_execution_date.and_time(parsed_time);
    println!("Word: {}", get_word().unwrap());
    loop {
        println!("Hello, clap! {:?} {}", args, next_execution_datetime);
        if next_execution_datetime <= Local::now().naive_local() {
            next_execution_datetime = (Local::now().date_naive()
                + chrono::Days::new(args.interval_days))
            .and_time(parsed_time);
            send_webhook(
                &args.webhook_url,
                &get_word().unwrap_or(
                    "There has been an error with getting todays' random prompt.".to_string(),
                ),
            )
            .unwrap();
        }
        sleep(Duration::new(5, 0));
    }
}

fn get_word() -> Result<String, reqwest::Error> {
    // Blocking here is okay, due to the very low frequency of requests.
    let resp = reqwest::blocking::get("https://random-word-form.repl.co/random/noun")?.text()?;
    Ok(from_str::<WordResponse>(&resp).unwrap().zero)
}

fn send_webhook(webhook_url: &str, message: &str) -> Result<(), reqwest::Error> {
    let payload = WebhookPayload {
        content: message.to_string(),
        embeds: None,
        attachments: vec![],
        username: "Artists' Random Prompt Generator".to_string(),
        avatar_url:
            "https://github.com/bitfl0wer/art-prompt-webhook/blob/main/static/icon.png?raw=true"
                .to_string(),
    };
    let client = reqwest::blocking::Client::new();
    dbg!(to_string(&payload).unwrap());
    let resp = client.post(webhook_url).header("Content-Type", "application/json").body(to_string(&payload).expect("There has been an error while serializing the Webhook Payload to JSON. Please report this bug at: https://github.com/bitfl0wer/art-prompt-webhook")).send()?;
    println!("Response: {:?}", resp.status());
    Ok(())
}
