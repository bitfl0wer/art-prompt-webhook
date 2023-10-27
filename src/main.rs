use clap::Parser;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    time: u16,
}

#[derive(Deserialize, Debug)]
struct WordResponse {
    #[serde(rename = "0")]
    zero: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, clap! {:?}", args);
}

fn get_word() -> Result<String, reqwest::Error> {
    let resp = reqwest::blocking::get("https://random-word-form.repl.co/random/noun")?.text()?;
    Ok(from_str::<WordResponse>(&resp).unwrap().zero)
}

#[test]
fn test() {
    println!("{}", get_word().unwrap());
}
