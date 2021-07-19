#!/usr/bin/env -S cargo eval --
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! twitch_api2 = {version = "0.5.0", features=["all"]}
//! twitch_oauth2 = { version = "0.5.0", features=["all"]}
//! reqwest = "*"
//! tokio = { version = "1.4.0", features = ["rt-multi-thread", "macros"] }
//! dotenv = "0.15.0"
//! surf = "*"
//! serde_json = "*"
//! configparser = "2.0.1"
//! shellexpand = "2.1"
//! ```
use std::io::stdout;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::process;

use twitch_oauth2::client::surf_http_client;
use twitch_oauth2::{AccessToken, RefreshToken, UserToken};

use twitch_api2::helix::streams::Stream;
use twitch_api2::{helix::streams::GetStreamsRequest, HelixClient};


use twitch_api2::helix::users::{GetUsersRequest, GetUsersFollowsRequest};
use twitch_api2::helix::users::get_users_follows::FollowRelationship;

const FIRST: usize = 100;

use configparser::ini::Ini;

async fn get_token() -> Result<UserToken, Error> {
    let mut config = Ini::new();

    let mut access_token = std::env::var("TWITCH_ACCESS_TOKEN").unwrap_or("".to_string());
    let mut refresh_token = std::env::var("TWITCH_REFRESH_TOKEN").unwrap_or("".to_string());
    let mut _client_id = std::env::var("TWITCH_CLIENT_ID").unwrap_or("".to_string());

    let conf_path = shellexpand::tilde("~/.config/twitch_dmenu/conf");

    if (access_token == "") | (refresh_token == "") {
        match config.load(&*conf_path) {
            Ok(_c) => {
                access_token = config.get("twitch-dmenu", "access_token").expect("");
                refresh_token = config.get("twitch-dmenu", "refresh_token").expect("");
                _client_id = config
                    .get("twitch-dmenu", "client_id")
                    .unwrap_or("".to_string());
            }
            Err(c) => panic!("Problem loading config and env vars not set: \n{}", c),
        };
    };

    // let _client_id = twitch_oauth2::ClientId::new(client_id);

    let token = UserToken::from_existing(
        surf_http_client,
        AccessToken::new(access_token),
        RefreshToken::new(refresh_token),
        None, // Client Secret
    )
    .await
    .unwrap();

    Ok(token)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let token = get_token().await.unwrap();
    let client_helix = HelixClient::with_client(surf::Client::new());

    let user_req = GetUsersRequest::builder().build();
    let user = client_helix.req_get(user_req, &token).await.unwrap();
    let self_user_id: String = user.first().unwrap().id;

    let req = GetUsersFollowsRequest::builder()
        .first(Some(FIRST))
        .from_id(self_user_id)
        .build();

    let response = client_helix.req_get(req, &token).await.unwrap();

    let followers: Vec<FollowRelationship> = response.data.follow_relationships;

    let mut followed_accounts = Vec::new();
    followers
        .iter()
        .for_each(|f| followed_accounts.push(f.to_id.clone()));

    let mut slice_len = FIRST;

    if followed_accounts.len() < FIRST {
        slice_len = followed_accounts.len()
    }

    let req = GetStreamsRequest::builder()
        .user_id(followed_accounts[1..slice_len].to_vec())
        .first(Some(slice_len))
        .build();

    let results: Vec<Stream> = client_helix.req_get(req, &token).await?.data;

    let mut stdout = stdout();

    for stream in results {
        let output_string = format!(
            "{:width$}|{viewers}| {title}",
            stream.user_login,
            viewers = stream.viewer_count,
            title = stream.title,
            width = 16
        );

        if let Err(e) = writeln!(stdout, "{}", &output_string) {
            if e.kind() != ErrorKind::BrokenPipe {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }

    Ok(())
}
