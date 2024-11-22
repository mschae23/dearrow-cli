use std::sync::Arc;
use anyhow::{Context, bail};
use chrono::DateTime;
use dearrow_browser_api::sync::{ApiThumbnail, ApiTitle};
use reqwest::Url;

use crate::{Options, SubmissionKind, OEmbedResponse, utils};

pub fn run(options: Options, client: reqwest::blocking::Client, terminal_width: u16, video: String, kind: SubmissionKind) -> anyhow::Result<()> {
    match kind {
        SubmissionKind::Title => {
            let url = Url::parse(&options.browser_api)?;
            let url = url.join("titles/video_id/")?.join(&video)?;

            let response = client.get(url).header("User-Agent", crate::USER_AGENT).send().context("Failed to send branding request")?;

            if response.status() != 200 {
                bail!("Failed to get titles. Response: {}\n{}", response.status(), response.text()?);
            }

            let mut titles: Vec<ApiTitle> = response.json()?;
            titles.sort_by(|a, b| a.time_submitted.cmp(&b.time_submitted).reverse());

            let url = Url::parse_with_params(
                "https://www.youtube-nocookie.com/oembed",
                &[("url", format!("https://youtu.be/{}", video))]
            ).context("Failed to construct an oembed request URL")?;
            let resp: OEmbedResponse = client.get(url).header("User-Agent", crate::USER_AGENT)
                .send().context("Failed to send oembed request")?
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

            for title in &titles {
                let mut flags = format!("{:>width$} ({:>+width$} | {})", title.score, title.votes,
                    if title.downvotes == 0 {
                        format!("{: >width$}-0", "", width = score_length.saturating_sub(2) as usize)
                    } else {
                        format!("{:->width$}", -title.downvotes, width = score_length as usize)
                    }, width = score_length as usize);

                let mut flag = false;
                flags.push_str(", ");

                if title.votes - title.downvotes < -1 {
                    flags.push_str("d"); // Removed by downvotes
                    flag = true;
                } else if title.votes < 0 {
                    flags.push_str("r"); // Replaced by submitter
                    flag = true;
                } else if title.score < 0 {
                    flags.push_str("h"); // Title should only appear in submission menus
                    flag = true;
                }

                if title.unverified {
                    flags.push_str("u"); // Submitted by unverified user
                    flag = true;
                }

                if title.locked {
                    flags.push_str("l"); // Locked by a VIP
                    flag = true;
                }

                if title.removed {
                    flags.push_str("m"); // Removed by VIP
                    flag = true;
                }

                if title.vip {
                    flags.push_str("v"); // Submitted by VIP
                    flag = true;
                }

                if title.shadow_hidden {
                    flags.push_str("x"); // Shadowhidden
                    flag = true;
                }

                if !flag {
                    flags.truncate(flags.len() - 2);
                }

                builder.push_record([
                    DateTime::from_timestamp_millis(title.time_submitted).map_or(title.time_submitted.to_string(), utils::render_datetime),
                    title.title.to_string(),
                    flags,
                    title.uuid.to_string(),
                    if let Some(username) = title.username.as_ref().map(Arc::clone) { format!("\"{}\"", username) } else { String::new() },
                    title.user_id.to_string(),
                ]);
            }

            let table_settings = tabled::settings::Settings::default()
                .with(tabled::settings::Style::psql())
                .with(tabled::settings::Width::wrap(terminal_width as usize).priority(tabled::settings::peaker::PriorityMax::new(false)))
                .with(tabled::settings::Width::increase(terminal_width as usize));

            let mut table = builder.build();
            table.with(table_settings);

            for (i, _) in titles.iter().enumerate() {
                table.modify(tabled::settings::object::Cell::new(i, 4),
                    tabled::settings::Width::truncate(16).suffix("..."));
            }

            println!("{}", table);
        },
        SubmissionKind::Thumbnail => {
            let url = Url::parse(&options.browser_api)?;
            let url = url.join("thumbnails/video_id/")?.join(&video)?;

            let response = client.get(url).header("User-Agent", crate::USER_AGENT).send().context("Failed to send branding request")?;

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
                    DateTime::from_timestamp_millis(thumbnail.time_submitted).map_or(thumbnail.time_submitted.to_string(), utils::render_datetime),
                    thumbnail.timestamp.map(|t| t.to_string()).unwrap_or_else(|| if thumbnail.original { String::from("Original") } else { String::from("Unknown") }),
                    flags,
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
            println!("{}", table);
        },
    }

    Ok(())
}