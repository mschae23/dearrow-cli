use anyhow::{Context, bail};
use chrono::DateTime;
use dearrow_browser_api::sync::{ApiWarning, Extension};
use reqwest::Url;

use crate::{Options, UserSubcommand, WarningKind, utils};

pub fn run(options: Options, client: reqwest::blocking::Client, terminal_width: u16, user: String, subcommand: UserSubcommand) -> anyhow::Result<()> {
    match subcommand {
        UserSubcommand::Warnings { kind, newest, } => {
            let issued = match kind { WarningKind::Issued => true, WarningKind::Received => false, };

            let url = Url::parse(&format!("{}warnings/user_id/{}/{}", &options.browser_api, user, kind.name()))?;
            let response = client.get(url).header("User-Agent", crate::USER_AGENT).send().context("Failed to send branding request")?;

            if response.status() != 200 {
                bail!("Failed to get thumbnails. Response: {}\n{}", response.status(), response.text()?);
            }

            let warnings: Vec<ApiWarning> = response.json()?;
            let warnings_len = warnings.len();

            let mut builder = tabled::builder::Builder::new();
            builder.push_record(["Message", "Timestamp", "Extension", "Active", if issued { "Warned" } else { "Issuer" },]);

            for warning in warnings.into_iter().take(if newest != 0 { newest } else { warnings_len }) {
                builder.push_record([
                    warning.message.to_string(),
                    DateTime::from_timestamp_millis(warning.time_issued).map_or(warning.time_issued.to_string(), utils::render_datetime),
                    match warning.extension { Extension::SponsorBlock => String::from("SB"), Extension::DeArrow => String::from("DeArrow"), },
                    warning.active.to_string(),
                    if issued { warning.warned_user_id.to_string() } else { warning.issuer_user_id.to_string() },
                ]);
            }

            let table_settings = tabled::settings::Settings::default()
                .with(tabled::settings::Style::psql())
                .with(tabled::settings::Width::wrap(terminal_width as usize).priority(tabled::settings::peaker::PriorityLeft::new()));

            let table = builder.build().with(table_settings).to_string();
            println!("{}", table);

            Ok(())
        },
    }
}