use std::collections::HashMap;
use std::io::Write;
use std::process::ExitCode;
use clap::Parser;

const USER_AGENT: &'static str = "CLI script (mschae23)/1.0.0";

#[derive(clap::Parser)]
pub struct Config {
    #[arg(long = "skip", help = "How many records to skip initially", default_value = "0")]
    pub skip: u32,
}

fn main() -> ExitCode {
    let config = Config::parse();

    let private_user_id = match std::env::var("SPONSORBLOCK_PRIVATE_USERID") {
        Ok(var) => var,
        Err(err) => {
            eprintln!("Could not get private user ID: {}", err);
            return ExitCode::FAILURE;
        },
    };

    let mut request_data = HashMap::new();
    request_data.insert("service", serde_json::Value::String(String::from("YouTube")));
    request_data.insert("userAgent", serde_json::Value::String(String::from(USER_AGENT)));
    request_data.insert("userID", serde_json::Value::String(String::from(&private_user_id)));

    let client = reqwest::blocking::Client::new();

    let mut reader = csv::Reader::from_path("/opt/clickbait-db-backup/old_submissions.csv").expect("Could not create CSV reader");
    println!("Reading input CSV file...");
    let stdin = std::io::stdin();
    let mut buf = String::new();

    for record in reader.records().skip(config.skip as usize) {
        match record {
            Ok(record) => {
                println!();

                if record.len() != 2 {
                    eprintln!("Skipping record. Error: CSV record does not have two fields ({})", record.len());
                } else {
                    let video_id = &record[0];
                    let original_title = &record[1];

                    print!("[Video ID: {}] {}\nNew title: ", video_id, original_title);
                    std::io::stdout().flush().expect("Could not flush stdout");

                    buf.clear();
                    match stdin.read_line(&mut buf) {
                        Ok(_) => {
                            let trimmed_input = buf.trim();

                            if trimmed_input.is_empty() {
                                eprintln!("Skipping record.");
                            } else {
                                request_data.insert("videoID", serde_json::Value::String(String::from(video_id)));
                                request_data.insert("title", serde_json::Value::Object([
                                    (String::from("title"), serde_json::Value::String(String::from(trimmed_input)))
                                ].into_iter().collect()));

                                match client.post("https://sponsor.ajay.app/api/branding")
                                    .json(&request_data)
                                    .send() {
                                    Ok(response) => println!("Sent request. Response: {}", response.status()),
                                    Err(err) => eprintln!("Error sending request: {}", err),
                                }
                            }
                        },
                        Err(err) => eprintln!("Skipping record. Error: {}", err),
                    }
                }
            },
            Err(err) => eprintln!("Skipping record. Error: {}", err),
        }
    }

    return ExitCode::SUCCESS;
}
