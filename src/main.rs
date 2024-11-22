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

use std::path::PathBuf;
use clap::{Parser, Args};
use clap::crate_version;

mod command;

const USER_AGENT: &str = concat!("dearrow-cli/", crate_version!());

mod utils {
    use chrono::{DateTime, Utc};

    const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn render_datetime(dt: DateTime<Utc>) -> String {
        format!("{}", dt.format(TIME_FORMAT))
    }
}

/// A CLI program to view and vote for DeArrow submissions.
#[derive(clap::Parser)]
#[command(version, about, long_about = "A CLI program to view and vote for DeArrow submissions.")]
pub struct Config {
    /// The action to do
    #[command(subcommand)]
    pub verb: Verb,
    #[command(flatten)]
    pub options: Options,
}

#[derive(Args)]
pub struct Options {
    /// The URI base for the main voting and query commands. It has to be compatible
    /// with the DeArrow API (SponsorBlockServer).
    ///
    /// The value provided will be concatenated with the API path to form the final URI used.
    /// Note that the trailing slash is significant.
    #[arg(long, default_value = "https://sponsor.ajay.app/api/")]
    pub main_api: String,
    /// The URI base of the API used for the database browser commands. It has to be
    /// compatible with the internal API of DeArrowBrowser.
    ///
    /// The value provided will be concatenated with the API path to form the final URI used.
    /// Note that the trailing slash is significant.
    #[arg(long, default_value = "https://dearrow.minibomba.pro/api/")]
    pub browser_api: String,
}

#[derive(clap::Subcommand)]
pub enum Verb {
    /// Vote for a DeArrow submission on a video.
    ///
    /// This supports both upvotes and downvotes for titles and thumbnails.
    #[command()]
    Vote {
        /// ID of the video to vote for a submission on.
        #[arg(value_name = "VIDEO_ID")]
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
        #[arg(value_name = "VIDEO_ID")]
        video: String,
        /// The kind of submissions to show.
        #[arg(value_enum)]
        kind: SubmissionKind,
    },
    /// View information about a specific user.
    #[command()]
    User {
        /// The public ID of the user to look up information for.
        #[arg(value_name = "USER_ID")]
        user: String,
        /// The kind of information to query about the user.
        #[command(subcommand)]
        subcommand: UserSubcommand,
    },
    #[command(hide = true)]
    Batch {
        #[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
        input: PathBuf,
        /// When set, disables auto-lock (only has an effect for VIP users).
        ///
        /// Disabling auto-vote makes the vote count like it would coming from a normal user,
        /// which means:
        /// - A new submission is not locked by default
        /// - Voting for an existing submission will increment its score, but not lock it
        /// - Downvoting an existing submission will decrement its score, but not immediately remove it
        #[arg(long, short = 'n', help = "When set, disables auto-lock (only has an effect for VIP users)", long_help = "When set, disables auto-lock (only has an effect for VIP users).\n\nDisabling auto-vote makes the vote count like it would coming from a normal user, which means:\n- A new submission is not locked by default\n- Voting for an existing submission will increment its score, but not lock it\n- Downvoting an existing submission will decrement its score, but not immediately remove it")]
        no_autolock: bool,
        /// When set, requests to the DeArrow server will be printed instead of sent.
        #[arg(short, long)]
        simulate: bool,
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
        /// Whether to report this title submission as having been auto-warned.
        ///
        /// This is intended to log potentially low-quality submissions to DeArrow moderators.
        #[arg(long, help = "Whether to report this title submission as having been auto-warned", long_help = "Whether to report this title submission as having been auto-warned.\n\nThis is intended to log potentially low-quality submissions to DeArrow moderators.")]
        was_warned: bool,
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

#[derive(clap::Subcommand)]
pub enum UserSubcommand {
    /// View warnings associated with this user.
    ///
    /// Supports both received and issued warnings.
    #[command()]
    Warnings {
        /// The kind of warnings to show.
        #[arg(value_enum)]
        kind: WarningKind,
        /// Only show the newest n warnings. Set to `0` to show all.
        #[arg(long, default_value = "0")]
        newest: usize,
    },
    // TODO Submissions
}

#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum WarningKind {
    /// View warnings issued by this user.
    #[value()]
    Issued,
    /// View warnings received by this user.
    #[value()]
    Received,
}

impl WarningKind {
    pub fn name(self: Self) -> &'static str {
        match self {
            Self::Issued => "issued",
            Self::Received => "received",
        }
    }
}

#[derive(serde::Deserialize)]
struct OEmbedResponse {
    title: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let client = reqwest::blocking::Client::new();
    let terminal_width = termsize::get().map(|size| size.cols).unwrap_or(120);

    match config.verb {
        Verb::Vote { kind, video, downvote, no_autolock } =>
            command::vote::run(config.options, client, terminal_width, kind, video, downvote, no_autolock),
        Verb::View { video, kind } =>
            command::view::run(config.options, client, terminal_width, video, kind),
        Verb::User { user, subcommand } =>
            command::user::run(config.options, client, terminal_width, user, subcommand),
        Verb::Batch { input, no_autolock, simulate } =>
            command::batch::run(config.options, client, terminal_width, input, no_autolock, simulate),
    }
}
