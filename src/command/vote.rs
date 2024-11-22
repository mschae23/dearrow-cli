use std::collections::HashMap;
use anyhow::{Context, anyhow};
use reqwest::Url;

use crate::{Options, ThumbnailSubmission, VoteSubmissionSubcommand};

pub fn run(options: Options, client: reqwest::blocking::Client, _terminal_width: u16, kind: VoteSubmissionSubcommand, video: String, downvote: bool, no_autolock: bool) -> anyhow::Result<reqwest::blocking::Response> {
    let private_user_id = std::env::var("SPONSORBLOCK_PRIVATE_USERID").context("Could not get private user ID")?;

    let mut request_data = HashMap::new();
    request_data.insert("service", serde_json::Value::String(String::from("YouTube")));
    request_data.insert("userAgent", serde_json::Value::String(String::from(crate::USER_AGENT)));
    request_data.insert("userID", serde_json::Value::String(String::from(&private_user_id)));
    request_data.insert("videoID", serde_json::Value::String(video));
    request_data.insert("downvote", serde_json::Value::Bool(downvote));
    request_data.insert("autoLock", serde_json::Value::Bool(!no_autolock));

    match kind {
        VoteSubmissionSubcommand::Title { title, was_warned, } => {
            request_data.insert("title", serde_json::Value::Object([
                (String::from("title"), serde_json::Value::String(title)),
            ].into_iter().collect()));
            request_data.insert("wasWarned", serde_json::Value::Bool(was_warned));
        },
        VoteSubmissionSubcommand::Thumbnail { thumbnail, } => {
            match thumbnail {
                ThumbnailSubmission::Original {} => {
                    request_data.insert("thumbnail", serde_json::Value::Object([
                        (String::from("original"), serde_json::Value::Bool(true)),
                    ].into_iter().collect()));
                },
                ThumbnailSubmission::At { timestamp, } => {
                    request_data.insert("thumbnail", serde_json::Value::Object([
                        (String::from("original"), serde_json::Value::Bool(false)),
                        (String::from("timestamp"), serde_json::Value::Number(serde_json::Number::from_f64(timestamp).ok_or_else(|| anyhow!("Cannot parse timestamp"))?)),
                    ].into_iter().collect()));
                },
            }
        },
    }

    let url = Url::parse(&options.main_api)?;
    let url = url.join("branding")?;

    let response = client.post(url)
        .header("User-Agent", crate::USER_AGENT)
        .json(&request_data)
        .send().context("Failed to send branding request")?;
    eprintln!("Sent request. Response: {}", response.status());

    response.error_for_status().context("Server returned error")
}
