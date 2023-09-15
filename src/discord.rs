use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::env;

use crate::models::{AssetType, Changelog, Client};

pub async fn send_message(changelog: &Changelog, client: &Client) -> Result<(), SerenityError> {
    let http = Http::new("token");
    let token = env::var("CHANGELOG_WEBHOOK_URL").expect("missing CHANGELOG_WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let fail_safe_asset = String::from("https://discord.com/");
    let asset = changelog.asset.as_ref().unwrap_or_else(|| &fail_safe_asset);

    let image = match &changelog.asset_type {
        AssetType::Image | AssetType::Unknown => asset.to_string(),
        AssetType::YouTube => format!("https://i.ytimg.com/vi/{}/maxresdefault.jpg", &asset),
    };

    let url = match &changelog.asset_type {
        AssetType::Image | AssetType::Unknown => asset.to_string(),
        AssetType::YouTube => format!("https://youtu.be/{}", &asset),
    };

    let changelog_embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(135, 134, 255))
            .field("Changelog Id", &changelog.changelog_id, true)
            .field("Entry Id", &changelog.entry_id, true)
            .field("Asset Type", changelog.asset_type.clone() as u8, true)
            .description(format!(
                "{}\n\n**__Asset Link__**\n{}",
                &changelog.content, url,
            ))
            .footer(|f| f.text(format!("{} • {}", changelog.date, changelog.locale)))
            .image(image)
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
