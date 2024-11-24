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

use std::io::Write;
use anyhow::{Context, bail};
use chrono::DateTime;
use reqwest::Url;
use serde::Deserialize;
use dearrow_browser_api::string::{ApiThumbnail, ApiTitle};

use crate::{Options, SubmissionKind, OEmbedResponse, utils};

#[derive(Deserialize, Clone, PartialEq, Eq)]
pub struct MainApiTitle {
    pub title: String,
    pub original: bool,
    pub votes: i32,
    pub locked: bool,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "userID")]
    pub user_id: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct MainApiThumbnail {
    pub timestamp: Option<f64>,
    pub original: bool,
    pub votes: i32,
    pub locked: bool,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "userID")]
    pub user_id: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct MainApiResponse {
    pub titles: Vec<MainApiTitle>,
    pub thumbnails: Vec<MainApiThumbnail>,

    #[serde(rename = "randomTime")]
    pub random_time: Option<f64>,
    #[serde(rename = "videoDuration")]
    pub video_duration: Option<f64>,
}

fn get_original_title(client: &reqwest::blocking::Client, video: &str) -> anyhow::Result<String> {
    let url = Url::parse_with_params(
        "https://www.youtube-nocookie.com/oembed",
        &[("url", format!("https://youtu.be/{}", video))]
    ).context("Failed to construct an oembed request URL")?;
    let resp: OEmbedResponse = client.get(url).header("User-Agent", crate::USER_AGENT)
        .send().context("Failed to send oembed request")?
        .json().context("Failed to deserialize oembed response")?;
    resp.title.context("oembed response contained no title")
}

fn print_header(client: &reqwest::blocking::Client, video: &str, writer: &mut impl std::io::Write) -> anyhow::Result<()> {
    Ok(write!(writer, "View on YouTube: https://www.youtube.com/watch?v={}\nUses DeArrow data licensed under CC BY-NC-SA 4.0 from https://dearrow.ajay.app/.\nOriginal title: {}\n",
        video, get_original_title(client, video).context("Failed to get original title")?)?)
}

fn make_request(client: &reqwest::blocking::Client, url: Url) -> anyhow::Result<reqwest::blocking::Response> {
    let response = client.get(url).header("User-Agent", crate::USER_AGENT).send().context("Failed to send request")?;

    if response.status() != 200 {
        bail!("Failed to get submissions. Response: {}\n{}", response.status(), response.text()?);
    }

    Ok(response)
}

pub fn run(options: Options, client: reqwest::blocking::Client, terminal_width: u16, video: String, kind: SubmissionKind) -> anyhow::Result<()> {
    match kind {
        SubmissionKind::Main => {
            let url = Url::parse(&format!("{}branding?returnUserID=true&fetchAll=true&videoID={}",
                &options.main_api, &video))?;
            let response = make_request(&client, url)?;

            let response: MainApiResponse = response.json()?;
            let _titles_len = response.titles.len();

            let mut stdout = std::io::stdout();
            print_header(&client, &video, &mut stdout)?;

            if let Some(video_duration) = response.video_duration {
                write!(stdout, "Video duration: {}\n", video_duration)?;
            }

            if let Some(random_time) = response.random_time {
                if let Some(video_duration) = response.video_duration {
                    write!(stdout, "Random time: {} (timestamp: {})\n", random_time, random_time * video_duration)?;
                } else {
                    write!(stdout, "Random time: {}\n", random_time)?;
                }
            }

            let mut titles_builder = tabled::builder::Builder::new();
            let mut thumbnails_builder = tabled::builder::Builder::new();
            titles_builder.push_record(["Title", "Score", "UUID", "User ID"]);
            thumbnails_builder.push_record(["Thumbnail", "Score", "UUID", "User ID"]);

            let mut score_length = 1;

            for title in &response.titles {
                score_length = score_length.max(if title.votes == 0 { 1 } else { title.votes.abs().ilog10() + 1 + (title.votes.is_negative() as u32) });
            }

            for thumbnail in &response.thumbnails {
                score_length = score_length.max(if thumbnail.votes == 0 { 1 } else { thumbnail.votes.abs().ilog10() + 1 + (thumbnail.votes.is_negative() as u32) });
            }

            for title in response.titles {
                let mut score = format!("{:>width$}", title.votes, width = score_length as usize);
                let mut flags = String::new();

                if title.original {
                    flags.push('o');
                }

                if !title.locked && title.votes < 0 {
                    flags.push('h'); // Title should only appear in submission menus
                }

                if title.locked {
                    flags.push('l'); // Locked by a VIP
                }

                if !flags.is_empty() {
                    score.reserve(2 + flags.len());
                    score.push_str(", ");
                    score.push_str(&flags);
                }

                titles_builder.push_record([
                    title.title.to_string(),
                    score,
                    title.uuid.to_string(),
                    title.user_id.to_string(),
                ]);
            }

            for thumbnail in response.thumbnails {
                let mut score = format!("{:>width$}", thumbnail.votes, width = score_length as usize);
                let mut flags = String::new();

                if thumbnail.original {
                    flags.push('o');
                }

                if !thumbnail.locked && thumbnail.votes < 0 {
                    flags.push('h'); // Title should only appear in submission menus
                }

                if thumbnail.locked {
                    flags.push('l'); // Locked by a VIP
                }

                if !flags.is_empty() {
                    score.reserve(2 + flags.len());
                    score.push_str(", ");
                    score.push_str(&flags);
                }

                thumbnails_builder.push_record([
                    if let Some (timestamp) = thumbnail.timestamp { timestamp.to_string() } else { String::from("Original") },
                    score,
                    thumbnail.uuid.to_string(),
                    thumbnail.user_id.to_string(),
                ]);
            }

            let table_settings = tabled::settings::Settings::default()
                .with(tabled::settings::Style::psql())
                .with(tabled::settings::Width::wrap(terminal_width as usize).priority(tabled::settings::peaker::PriorityMax::new(false)))
                .with(tabled::settings::Width::increase(terminal_width as usize));

            let titles_table = titles_builder.build().with(table_settings.clone()).to_string();
            let thumbnails_table = thumbnails_builder.build().with(table_settings).to_string();

            write!(stdout, "\n{}\n\n{}\n", titles_table, thumbnails_table)?;
        },
        SubmissionKind::Title => {
            let url = Url::parse(&options.browser_api)?.join("titles/video_id/")?.join(&video)?;
            let response = make_request(&client, url)?;

            let mut titles: Vec<ApiTitle> = response.json()?;
            let titles_len = titles.len();
            titles.sort_by(|a, b| a.time_submitted.cmp(&b.time_submitted).reverse());

            let mut stdout = std::io::stdout();
            print_header(&client, &video, &mut stdout)?;

            let mut builder = tabled::builder::Builder::new();
            builder.push_record(["Submitted", "Title", "Score", "UUID", "Username", "User ID"]);

            let mut score_length = 1;

            for title in &titles {
                score_length = score_length.max(if title.score == 0 { 1 } else { title.score.abs().ilog10() + 1 + (title.score.is_negative() as u32) })
                    .max(if title.votes == 0 { 1 } else { title.votes.abs().ilog10() + 1 })
                    .max(if title.downvotes == 0 { 1 } else { title.downvotes.abs().ilog10() + 1 });
            }

            for title in titles {
                let mut score = format!("{:>width$} ({:>+width$} | {})", title.score, title.votes,
                    if title.downvotes == 0 {
                        format!("{: >width$}-0", "", width = score_length.saturating_sub(2) as usize)
                    } else {
                        format!("{:->width$}", -title.downvotes, width = score_length as usize)
                    }, width = score_length as usize);

                let mut flags = String::new();

                if title.original {
                    flags.push('o');
                }

                if title.removed || title.shadow_hidden {
                    if title.removed {
                        flags.push('m'); // Removed by VIP
                    }

                    if title.shadow_hidden {
                        flags.push('x'); // Shadowhidden
                    }
                } else if title.votes - title.downvotes < -1 {
                    flags.push('d'); // Removed by downvotes
                } else if title.votes < 0 {
                    flags.push('r'); // Replaced by submitter
                } else if !title.locked && title.score < 0 {
                    flags.push('h'); // Title should only appear in submission menus
                }

                if title.unverified {
                    flags.push('u'); // Submitted by unverified user
                }

                if title.locked {
                    flags.push('l'); // Locked by a VIP
                }

                if title.vip {
                    flags.push('v'); // Submitted by VIP
                }

                if !flags.is_empty() {
                    score.reserve(2 + flags.len());
                    score.push_str(", ");
                    score.push_str(&flags);
                }

                builder.push_record([
                    DateTime::from_timestamp_millis(title.time_submitted).map_or(title.time_submitted.to_string(), utils::render_datetime),
                    title.title.to_string(),
                    score,
                    title.uuid.to_string(),
                    if let Some(username) = title.username { format!("\"{}\"", username) } else { String::new() },
                    title.user_id.to_string(),
                ]);
            }

            let table_settings = tabled::settings::Settings::default()
                .with(tabled::settings::Style::psql())
                .with(tabled::settings::Width::wrap(terminal_width as usize).priority(tabled::settings::peaker::PriorityMax::new(false)))
                .with(tabled::settings::Width::increase(terminal_width as usize));

            let mut table = builder.build();
            table.with(table_settings);

            for i in 0..titles_len {
                table.modify(tabled::settings::object::Cell::new(i, 4),
                    tabled::settings::Width::truncate(16).suffix("..."));
            }

            write!(stdout, "\n{}\n", table)?;
        },
        SubmissionKind::Thumbnail => {
            let url = Url::parse(&options.browser_api)?.join("thumbnails/video_id/")?.join(&video)?;
            let response = make_request(&client, url)?;

            let mut thumbnails: Vec<ApiThumbnail> = response.json()?;
            thumbnails.sort_by(|a, b| a.time_submitted.cmp(&b.time_submitted).reverse());

            let mut stdout = std::io::stdout();
            print_header(&client, &video, &mut stdout)?;

            let mut builder = tabled::builder::Builder::new();
            builder.push_record(["Submitted", "Timestamp", "Score", "UUID", "Username", "User ID"]);

            let mut score_length = 1;

            for title in &thumbnails {
                score_length = score_length.max(if title.score == 0 { 1 } else { title.score.abs().ilog10() + 1 + (title.score.is_negative() as u32) })
                    .max(if title.votes == 0 { 1 } else { title.votes.abs().ilog10() + 1 })
                    .max(if title.downvotes == 0 { 1 } else { title.downvotes.abs().ilog10() + 1 });
            }

            for thumbnail in thumbnails {
                let mut score = format!("{:>width$} ({:>+width$} | {})", thumbnail.score, thumbnail.votes,
                    if thumbnail.downvotes == 0 {
                        format!("{: >width$}-0", "", width = score_length.saturating_sub(2) as usize)
                    } else {
                        format!("{:->width$}", -thumbnail.downvotes, width = score_length as usize)
                    }, width = score_length as usize);

                let mut flags = String::new();

                if thumbnail.removed || thumbnail.shadow_hidden {
                    if thumbnail.removed {
                        flags.push('m'); // Removed by VIP
                    }

                    if thumbnail.shadow_hidden {
                        flags.push('x'); // Shadowhidden
                    }
                } else if thumbnail.votes - thumbnail.downvotes < -1 {
                    flags.push('d'); // Removed by downvotes
                } else if !thumbnail.locked {
                    if (thumbnail.original && thumbnail.score < 1) || thumbnail.score < 0 {
                        // if original: Thumbnail has insufficient score to be shown (needs >=1 or lock)
                        // if not:      Thumbnail should only appear in submission menus
                        flags.push('h');
                    }
                }

                if thumbnail.locked {
                    flags.push('l'); // Locked by a VIP
                }

                if thumbnail.vip {
                    flags.push('v'); // Submitted by VIP
                }

                if !flags.is_empty() {
                    score.reserve(2 + flags.len());
                    score.push_str(", ");
                    score.push_str(&flags);
                }

                builder.push_record([
                    DateTime::from_timestamp_millis(thumbnail.time_submitted).map_or(thumbnail.time_submitted.to_string(), utils::render_datetime),
                    thumbnail.timestamp.map(|t| t.to_string()).unwrap_or_else(|| if thumbnail.original { String::from("Original") } else { String::from("Unknown") }),
                    score,
                    thumbnail.uuid.to_string(),
                    if let Some(username) = thumbnail.username { format!("\"{}\"", username) } else { String::new() },
                    thumbnail.user_id.to_string(),
                ]);
            }

            let table_settings = tabled::settings::Settings::default()
                .with(tabled::settings::Style::psql())
                .with(tabled::settings::Width::wrap(terminal_width as usize).priority(tabled::settings::peaker::PriorityMax::new(false)))
                .with(tabled::settings::Width::increase(terminal_width as usize));

            let table = builder.build().with(table_settings).to_string();
            write!(stdout, "\n{}\n", table)?;
        },
    }

    Ok(())
}
