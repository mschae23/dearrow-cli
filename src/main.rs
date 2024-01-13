use std::collections::HashMap;
use std::io::Write;
use anyhow::anyhow;

const USER_AGENT: &str = "CLI script (mschae23)/2.0.0";

fn main() -> anyhow::Result<()> {
    let private_user_id = match std::env::var("SPONSORBLOCK_PRIVATE_USERID") {
        Ok(var) => var,
        Err(err) => {
            return Err(anyhow!("Could not get private user ID: {}", err));
        },
    };

    let mut request_data = HashMap::new();
    request_data.insert("service", serde_json::Value::String(String::from("YouTube")));
    request_data.insert("userAgent", serde_json::Value::String(String::from(USER_AGENT)));
    request_data.insert("userID", serde_json::Value::String(String::from(&private_user_id)));

    let client = reqwest::blocking::Client::new();

    let stdin = std::io::stdin();
    let mut buf = String::new();

    print!("Video ID: ");
    std::io::stdout().flush().expect("Could not flush stdout");
    stdin.read_line(&mut buf)?;
    buf.pop();
    let video_id = buf.clone();
    buf.clear();

    print!("Title: ");
    std::io::stdout().flush().expect("Could not flush stdout");
    stdin.read_line(&mut buf)?;
    buf.pop();
    let title = buf.clone();
    buf.clear();

    print!("Downvote (y/N): ");
    std::io::stdout().flush().expect("Could not flush stdout");
    stdin.read_line(&mut buf)?;
    buf.pop();

    let downvote = buf == "y" || buf == "Y";
    buf.clear();

    println!("{}", if downvote { "Downvoting." } else { "Upvoting." });
    print!("Auto-lock (Y/n): ");
    std::io::stdout().flush().expect("Could not flush stdout");
    stdin.read_line(&mut buf)?;
    buf.pop();

    let autolock = buf != "n" && buf != "N";
    buf.clear();

    request_data.insert("videoID", serde_json::Value::String(video_id));
    request_data.insert("title", serde_json::Value::Object([
        (String::from("title"), serde_json::Value::String(title))
    ].into_iter().collect()));
    request_data.insert("downvote", serde_json::Value::Bool(downvote));
    request_data.insert("autoLock", serde_json::Value::Bool(autolock));

    match client.post("https://sponsor.ajay.app/api/branding")
        .json(&request_data)
        .send() {
        Ok(response) => println!("Sent request. Response: {}", response.status()),
        Err(err) => eprintln!("Error sending request: {}", err),
    }

    Ok(())
}
