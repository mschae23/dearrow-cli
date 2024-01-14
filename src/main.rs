// dearrow-cli - program to vote for DeArrow submissions
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

use std::collections::HashMap;
use anyhow::{anyhow, bail, Context};
use chrono::NaiveDateTime;
use clap::Parser;
use dearrow_browser_api::{ApiThumbnail, ApiTitle};
use reqwest::Url;

const USER_AGENT: &str = "CLI script (mschae23)/3.1.0";

mod utils {
    use chrono::NaiveDateTime;

    const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn render_naive_datetime(dt: NaiveDateTime) -> String {
        format!("{}", dt.format(TIME_FORMAT))
    }
}

#[derive(clap::Parser)]
pub struct Config {
    /// The action to do
    #[command(subcommand)]
    pub verb: Verb,
    /// The URI for the POST /branding API endpoint.
    #[arg(long, default_value = "https://sponsor.ajay.app/api/branding")]
    pub post_branding_api: String,
    /// A partial URI for the GET /titles API endpoint.
    ///
    /// The value provided will be concatenated with the video ID to form the final URI used.
    /// Note that the trailing slash is significant.
    #[arg(long, default_value = "https://dearrow.minibomba.pro/api/titles/video_id/")]
    pub get_titles_by_video_id_api: String,
    /// A partial URI for the GET /thumbnails API endpoint.
    ///
    /// The value provided will be concatenated with the video ID to form the final URI used.
    /// Note that the trailing slash is significant.
    #[arg(long, default_value = "https://dearrow.minibomba.pro/api/thumbnails/video_id/")]
    pub get_thumbnails_by_video_id_api: String,
}

#[derive(clap::Subcommand)]
pub enum Verb {
    /// Vote for a DeArrow submission on a video.
    ///
    /// This supports both upvotes and downvotes for titles and thumbnails.
    #[command()]
    Vote {
        /// ID of the video to vote for a submission on.
        #[arg(long, short, value_name = "VIDEO_ID")]
        video: String,
        /// The kind of a submission (title or thumbnail).
        #[command(subcommand)]
        kind: VoteSubmissionSubcommand,
        /// When set, downvotes instead of upvoting.
        #[arg(long, short)]
        downvote: bool,
        /// When set, disables auto-lock (only has an effect for VIP users).
        ///
        /// Disabling auto-vote makes the vote count like it would coming from a normal user,
        /// which means:
        /// - A new submission is not locked by default
        /// - Voting for an existing submission will increment its score, but not lock it
        /// - Downvoting an existing submission will decrement its score, but not immediately remove it
        #[arg(long, short = 'n', help = "When set, disables auto-lock (only has an effect for VIP users)", long_help = "When set, disables auto-lock (only has an effect for VIP users).\n\nDisabling auto-vote makes the vote count like it would coming from a normal user, which means:\n- A new submission is not locked by default\n- Voting for an existing submission will increment its score, but not lock it\n- Downvoting an existing submission will decrement its score, but not immediately remove it")]
        no_autolock: bool,
    },
    /// View DeArrow submissions on a video.
    #[command()]
    View {
        /// ID of the video to view submissions for.
        #[arg(long, short, value_name = "VIDEO_ID")]
        video: String,
        /// The kind of submissions to show..
        #[arg(value_enum)]
        kind: SubmissionKind,
    },
}

#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SubmissionKind {
    /// View title submissions.
    #[value()]
    Title,
    /// View thumbnail submissions.
    #[value()]
    Thumbnail,
}

#[derive(clap::Subcommand)]
pub enum VoteSubmissionSubcommand {
    /// Vote for a title submission.
    #[command()]
    Title {
        /// The title to vote for.
        #[arg()]
        title: String,
    },
    /// Vote for a thumbnail submission.
    #[command()]
    Thumbnail {
        /// The thumbnail to vote for.
        #[command(subcommand)]
        thumbnail: ThumbnailSubmission,
    },
}

#[derive(clap::Subcommand)]
pub enum ThumbnailSubmission {
    /// The original thumbnail chosen by the video's uploader.
    #[command()]
    Original {
    },
    /// A frame at a specific timestamp.
    #[command()]
    At {
        /// The timestamp to vote for.
        #[arg()]
        timestamp: f64,
    }
}

#[derive(serde::Deserialize)]
struct OEmbedResponse {
    title: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    match config.verb {
        Verb::Vote { kind, video, downvote, no_autolock } => {
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
            request_data.insert("videoID", serde_json::Value::String(video));
            request_data.insert("downvote", serde_json::Value::Bool(downvote));
            request_data.insert("autoLock", serde_json::Value::Bool(!no_autolock));

            match kind {
                VoteSubmissionSubcommand::Title { title, } => {
                    request_data.insert("title", serde_json::Value::Object([
                        (String::from("title"), serde_json::Value::String(title)),
                    ].into_iter().collect()));
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

            let client = reqwest::blocking::Client::new();
            let response = client.post(&config.post_branding_api)
                .json(&request_data)
                .send().context("Failed to send branding request")?;
            eprintln!("Sent request. Response: {}", response.status());

            Ok(())
        },
        Verb::View { video, kind } => {
            let client = reqwest::blocking::Client::new();
            let terminal_width = termsize::get().map(|size| size.cols).unwrap_or(120);

            match kind {
                SubmissionKind::Title => {
                    let url = Url::parse(&config.get_titles_by_video_id_api)?;
                    let url = url.join(&video)?;

                    let response = client.get(url).send().context("Failed to send branding request")?;

                    if response.status() != 200 {
                        bail!("Failed to get titles. Response: {}\n{}", response.status(), response.text()?);
                    }

                    let mut titles: Vec<ApiTitle> = response.json()?;
                    titles.sort_by(|a, b| a.time_submitted.cmp(&b.time_submitted).reverse());

                    let url = Url::parse_with_params(
                        "https://www.youtube-nocookie.com/oembed",
                        &[("url", format!("https://youtu.be/{}", video))]
                    ).context("Failed to construct an oembed request URL")?;
                    let resp: OEmbedResponse = client.get(url).send().context("Failed to send oembed request")?
                        .json().context("Failed to deserialize oembed response")?;
                    let original_title = resp.title.context("oembed response contained no title")?;

                    println!("View on YouTube: https://youtube.com/watch?v={}", video);
                    println!("Original title: {}", original_title);
                    println!("Uses DeArrow data licensed under CC BY-NC-SA 4.0 from https://dearrow.ajay.app/.");
                    println!();

                    let mut builder = tabled::builder::Builder::new();
                    builder.push_record(["Submitted", "Title", "Score", "UUID", "Username", "User ID"]);

                    let mut score_length = 1;

                    for title in &titles {
                        score_length = score_length.max(if title.score == 0 { 1 } else { title.score.abs().ilog10() + 1 + (title.score.is_negative() as u32) })
                            .max(if title.votes == 0 { 1 } else { title.votes.abs().ilog10() + 1 })
                            .max(if title.downvotes == 0 { 1 } else { title.downvotes.abs().ilog10() + 1 });
                    }

                    for title in titles {
                        let mut flags = format!("{:>width$} ({:>+width$} | {})", title.score, title.votes,
                            if title.downvotes == 0 {
                                format!("{: >width$}-0", "", width = score_length.saturating_sub(2) as usize)
                            } else {
                                format!("{:->width$}", -title.downvotes, width = score_length as usize)
                            }, width = score_length as usize);

                        if title.votes - title.downvotes < -1 {
                            flags.push_str(", d"); // Removed by downvotes
                        } else if title.votes < 0 {
                            flags.push_str(", r"); // Replaced by submitter
                        } else if title.score < 0 {
                            flags.push_str(", h"); // Title should only appear in submission menus
                        }

                        if title.unverified {
                            flags.push_str(", u"); // Submitted by unverified user
                        }

                        if title.locked {
                            flags.push_str(", l"); // Locked by a VIP
                        }

                        if title.removed {
                            flags.push_str(", rm"); // Removed by VIP
                        }

                        if title.vip {
                            flags.push_str(", v"); // Submitted by VIP
                        }

                        if title.shadow_hidden {
                            flags.push_str(", x"); // Shadowhidden
                        }

                        builder.push_record([
                            NaiveDateTime::from_timestamp_millis(title.time_submitted).map_or(title.time_submitted.to_string(), utils::render_naive_datetime),
                            title.title.to_string(),
                            flags,
                            title.uuid.to_string(),
                            if let Some(username) = title.username { format!("\"{}\"", username) } else { String::new() },
                            title.user_id.to_string(),
                        ]);
                    }

                    let table_settings = tabled::settings::Settings::default()
                        .with(tabled::settings::Style::psql())
                        .with(tabled::settings::Width::wrap(terminal_width as usize).priority::<tabled::settings::peaker::PriorityMax>())
                        .with(tabled::settings::Width::increase(terminal_width as usize));

                    let table = builder.build().with(table_settings).to_string();
                    println!("{}", table);
                },
                SubmissionKind::Thumbnail => {
                    let url = Url::parse(&config.get_thumbnails_by_video_id_api)?;
                    let url = url.join(&video)?;

                    let response = client.get(url).send().context("Failed to send branding request")?;

                    if response.status() != 200 {
                        bail!("Failed to get thumbnails. Response: {}\n{}", response.status(), response.text()?);
                    }

                    let mut thumbnails: Vec<ApiThumbnail> = response.json()?;
                    thumbnails.sort_by(|a, b| a.time_submitted.cmp(&b.time_submitted).reverse());

                    println!("View on YouTube: https://youtube.com/watch?v={}", video);
                    println!("Uses DeArrow data licensed under CC BY-NC-SA 4.0 from https://dearrow.ajay.app/.");
                    println!();

                    let mut builder = tabled::builder::Builder::new();
                    builder.push_record(["Submitted", "Timestamp", "Score", "UUID", "Username", "User ID"]);

                    let mut score_length = 1;

                    for title in &thumbnails {
                        score_length = score_length.max(if title.score == 0 { 1 } else { title.score.abs().ilog10() + 1 + (title.score.is_negative() as u32) })
                            .max(if title.votes == 0 { 1 } else { title.votes.abs().ilog10() + 1 })
                            .max(if title.downvotes == 0 { 1 } else { title.downvotes.abs().ilog10() + 1 });
                    }

                    for thumbnail in thumbnails {
                        let mut flags = format!("{:>width$} ({:>+width$} | {})", thumbnail.score, thumbnail.votes,
                            if thumbnail.downvotes == 0 {
                                format!("{: >width$}-0", "", width = score_length.saturating_sub(2) as usize)
                            } else {
                                format!("{:->width$}", -thumbnail.downvotes, width = score_length as usize)
                            }, width = score_length as usize);

                        if thumbnail.votes - thumbnail.downvotes < -1 {
                            flags.push_str(", d"); // Removed by downvotes
                        } else if thumbnail.score < 0 {
                            flags.push_str(", h"); // Title should only appear in submission menus
                        }

                        if thumbnail.locked {
                            flags.push_str(", l"); // Locked by a VIP
                        }

                        if thumbnail.removed {
                            flags.push_str(", rm"); // Removed by VIP
                        }

                        if thumbnail.vip {
                            flags.push_str(", v"); // Submitted by VIP
                        }

                        if thumbnail.shadow_hidden {
                            flags.push_str(", x"); // Shadowhidden
                        }

                        builder.push_record([
                            NaiveDateTime::from_timestamp_millis(thumbnail.time_submitted).map_or(thumbnail.time_submitted.to_string(), utils::render_naive_datetime),
                            thumbnail.timestamp.map(|t| t.to_string()).unwrap_or_else(|| if thumbnail.original { String::from("Original") } else { String::from("Unknown") }),
                            flags,
                            thumbnail.uuid.to_string(),
                            if let Some(username) = thumbnail.username { format!("\"{}\"", username) } else { String::new() },
                            thumbnail.user_id.to_string(),
                        ]);
                    }

                    let table_settings = tabled::settings::Settings::default()
                        .with(tabled::settings::Style::psql())
                        .with(tabled::settings::Width::wrap(terminal_width as usize).priority::<tabled::settings::peaker::PriorityMax>())
                        .with(tabled::settings::Width::increase(terminal_width as usize));

                    let table = builder.build().with(table_settings).to_string();
                    println!("{}", table);
                },
            }

            Ok(())
        },
    }
}
