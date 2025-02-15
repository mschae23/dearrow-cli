// dearrow-cli - program to view and vote for DeArrow submissions
// Copyright (C) 2024  mschae23
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use anyhow::{anyhow, Context};
use reqwest::Url;
use std::collections::HashMap;

use crate::{CasualCategory, Options, ThumbnailSubmission, VoteSubmissionSubcommand};

pub fn run(options: Options, client: reqwest::blocking::Client, _terminal_width: u16, kind: VoteSubmissionSubcommand, video: String, downvote: bool, no_autolock: bool, using_casual: bool) -> anyhow::Result<reqwest::blocking::Response> {
    let private_user_id = std::env::var("SPONSORBLOCK_PRIVATE_USERID").context("Could not get private user ID")?;

    let mut request_data = HashMap::new();
    request_data.insert("service", serde_json::Value::String(String::from("YouTube")));
    request_data.insert("userAgent", serde_json::Value::String(String::from(crate::USER_AGENT)));
    request_data.insert("userID", serde_json::Value::String(String::from(&private_user_id)));
    request_data.insert("videoID", serde_json::Value::String(video));
    request_data.insert("downvote", serde_json::Value::Bool(downvote));

    let is_casual = matches!(&kind, &VoteSubmissionSubcommand::Casual { .. });

    if !is_casual {
        request_data.insert("autoLock", serde_json::Value::Bool(!no_autolock));
        request_data.insert("casualMode", serde_json::Value::Bool(using_casual));
    }

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
        VoteSubmissionSubcommand::Casual { categories } => {
            if downvote {
                request_data.insert("categories", serde_json::Value::Array(vec![]));
            } else {
                request_data.insert("categories", serde_json::Value::Array(categories.into_iter()
                    .map(CasualCategory::name).map(String::from).map(serde_json::Value::String).collect()));
            }
        },
    }

    let url = Url::parse(&options.main_api)?;
    let url = url.join(if is_casual { "casual" } else { "branding" })?;

    let response = client.post(url)
        .header("User-Agent", crate::USER_AGENT)
        .json(&request_data)
        .send().context("Failed to send branding request")?;
    eprintln!("Sent request. Response: {}", response.status());

    response.error_for_status().context("Server returned error")
}
