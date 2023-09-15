mod discord;
mod models;
mod utils;

use std::{collections::HashMap, env};

use dotenv::dotenv;
use rusqlite::{Connection, Result};

use crate::{
    discord::send_message,
    models::{Changelog, ChangelogConfig, ChangelogDB, ChangelogReqwest},
    utils::which_client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let conn = Connection::open(env::var("DB_URL").expect("missing DB_URL in .env"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS changelogs (changelog_id TEXT PRIMARY KEY, client TEXT , locale TEXT, date TEXT, asset TEXT, asset_type INTEGER, content TEXT)",
        (),
    )?;

    for client_id in 0..2 {
        let client = which_client(client_id);
        let config_url = format!(
            "https://cdn.discordapp.com/changelogs/config_{}.json",
            client_id
        );
        println!("[{:#?}] Fetching config url: {}", client, config_url);

        let config_body = reqwest::get(&config_url).await?.text().await?;
        let config: HashMap<String, ChangelogConfig> = serde_json::from_str(&config_body)?;

        for (snowflake, value) in config {
            let min_version = value.min_version;

            let sql_select = format!(
                "SELECT * FROM changelogs WHERE changelog_id = '{}' AND client = '{}'",
                snowflake, client_id
            );
            let mut stmt = conn.prepare(&sql_select)?;
            let changelog_db = stmt.query_map([], |row| {
                Ok(ChangelogDB {
                    changelog_id: row.get(0)?,
                    client: row.get(1)?,
                    locale: row.get(2)?,
                    date: row.get(3)?,
                    asset: row.get(4)?,
                    asset_type: row.get(5)?,
                    content: row.get(6)?,
                })
            })?;

            if changelog_db.count() > 0 {
                println!("[{:#?}] Still on {}", client, snowflake);
                continue;
            }

            println!(
                "[{:#?}] Fetching the changelog for {} with min version of {}",
                client, snowflake, min_version
            );

            let changelog_url = format!(
                "https://cdn.discordapp.com/changelogs/{}/{}/en-US.json",
                client_id, snowflake
            );
            let changelog_req = reqwest::get(&changelog_url)
                .await?
                .json::<ChangelogReqwest>()
                .await?;

            let changelog = Changelog::convert_from_reqwest(changelog_req);

            println!("[{:#?}] Found changelog: {}", client, changelog.date);
            send_message(&changelog, &client).await?;

            conn.execute(
            "INSERT INTO changelogs (changelog_id, client, locale, date, asset, asset_type, content) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
               &changelog.changelog_id,
               client_id,
               &changelog.locale,
               &changelog.date,
               &changelog.asset,
               changelog.asset_type as u8,
               &changelog.content,
            ),
        )?;
        }
    }

    Ok(())
}
