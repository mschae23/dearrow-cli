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

use std::{collections::HashMap, fs::File, path::PathBuf};
use anyhow::{Context, anyhow};
use reqwest::Url;

use crate::{Options, OEmbedResponse};

pub fn run(options: Options, client: reqwest::blocking::Client, _terminal_width: u16, input: PathBuf, no_autolock: bool, simulate: bool) -> anyhow::Result<()> {
    let private_user_id = std::env::var("SPONSORBLOCK_PRIVATE_USERID").context("Failed to get private user ID")?;

    let mut request_data = HashMap::new();
    request_data.insert("service", serde_json::Value::String(String::from("YouTube")));
    request_data.insert("userAgent", serde_json::Value::String(String::from(crate::USER_AGENT)));
    request_data.insert("userID", serde_json::Value::String(String::from(&private_user_id)));
    request_data.insert("autoLock", serde_json::Value::Bool(!no_autolock));

    let file = File::open(input).context("Failed to open input file")?;
    let reader = std::io::BufReader::new(file);

    let stdin = std::io::stdin();
    let mut buf = String::new();
    let mut reader = csv::Reader::from_reader(reader);

    for record in reader.records() {
        let record = record.context("Failed to read line in input file")?;
        let video_id = record.get(0).ok_or(anyhow!("Failed to get column 0 from CSV record"))?;
        let old_title = record.get(1).ok_or(anyhow!("Failed to get column 1 from CSV record"))?;

        let url = Url::parse_with_params(
            "https://www.youtube-nocookie.com/oembed",
            &[("url", format!("https://youtu.be/{}", video_id))]
        ).context("Failed to construct an oembed request URL")?;
        let resp: OEmbedResponse = client.get(url).header("User-Agent", crate::USER_AGENT)
            .send().context("Failed to send oembed request")?
            .json().context("Failed to deserialize oembed response")?;
        let original_title = resp.title.context("oembed response contained no title")?;

        eprintln!("[{}, {}] {}", video_id, original_title, old_title);
        stdin.read_line(&mut buf).context("Failed to read stdin")?;

        if buf == "\n" {
            buf.clear();
            eprintln!("Skipped.\n");
            continue;
        }

        request_data.insert("videoID", serde_json::Value::String(video_id.to_owned()));
        request_data.insert("title", serde_json::Value::Object([
            (String::from("title"), serde_json::Value::String(buf[..buf.len() - 1].to_string())),
        ].into_iter().collect()));

        if !simulate {
            let url = Url::parse(&options.main_api)?.join("branding")?;

            let response = client.post(url)
                .header("User-Agent", crate::USER_AGENT)
                .json(&request_data)
                .send().context("Failed to send branding request")?;
            eprintln!("Sent request. Response: {}\n", response.status());
        } else {
            eprintln!("Not sending request: {}\n", serde_json::to_string_pretty(&request_data).context("Failed to serialize request to JSON")?);
        }

        buf.clear();
    }

    Ok(())
}