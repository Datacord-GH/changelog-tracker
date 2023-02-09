use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::env;

use crate::models::{Changelog, Client};

pub async fn send_message(changelog: &Changelog, client: &Client) -> Result<(), SerenityError> {
    let http = Http::new("token");
    let token = env::var("CHANGELOG_WEBHOOK_URL").expect("missing CHANGELOG_WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;
    let fail_safe_asset = String::from("https://cdn.discordapp.com/embed/avatars/0.png");

    let changelog_embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(135, 134, 255))
            .field("Changelog Id", &changelog.changelog_id, true)
            .field("Entry Id", &changelog.entry_id, true)
            .field("Asset Type", changelog.asset_type, true)
            .description(&changelog.content)
            .footer(|f| f.text(format!("{} • {}", changelog.date, changelog.locale)))
            .image(&changelog.asset.as_ref().unwrap_or_else(|| &fail_safe_asset))
            .title(format!("Changelog on {:#?}", client))
    });

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("Changelog Manager")
            .embeds(vec![changelog_embed])
        })
        .await?;

    Ok(())
}
