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
use anyhow::anyhow;
use clap::Parser;

const USER_AGENT: &str = "CLI script (mschae23)/3.0.0";

#[derive(clap::Parser)]
pub struct Config {
    /// The action to do
    #[command(subcommand)]
    pub verb: Verb,
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
        kind: SubmissionKind,
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
}

#[derive(clap::Subcommand)]
pub enum SubmissionKind {
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

fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let private_user_id = match std::env::var("SPONSORBLOCK_PRIVATE_USERID") {
        Ok(var) => var,
        Err(err) => {
            return Err(anyhow!("Could not get private user ID: {}", err));
        },
    };

    match config.verb {
        Verb::Vote { kind, video, downvote, no_autolock } => {
            let mut request_data = HashMap::new();
            request_data.insert("service", serde_json::Value::String(String::from("YouTube")));
            request_data.insert("userAgent", serde_json::Value::String(String::from(USER_AGENT)));
            request_data.insert("userID", serde_json::Value::String(String::from(&private_user_id)));
            request_data.insert("videoID", serde_json::Value::String(video));
            request_data.insert("downvote", serde_json::Value::Bool(downvote));
            request_data.insert("autoLock", serde_json::Value::Bool(!no_autolock));

            match kind {
                SubmissionKind::Title { title, } => {
                    request_data.insert("title", serde_json::Value::Object([
                        (String::from("title"), serde_json::Value::String(title)),
                    ].into_iter().collect()));
                },
                SubmissionKind::Thumbnail { thumbnail, } => {
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
            match client.post("https://sponsor.ajay.app/api/branding")
                .json(&request_data)
                .send() {
                Ok(response) => eprintln!("Sent request. Response: {}", response.status()),
                Err(err) => eprintln!("Error sending request: {}", err),
            }

            Ok(())
        },
    }
}
