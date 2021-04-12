use std::io::Error;

use twitch_oauth2::client::surf_http_client;
use twitch_oauth2::{AccessToken, RefreshToken, UserToken};

use twitch_api2::helix::streams::Stream;
use twitch_api2::{helix::streams::GetStreamsRequest, HelixClient};

use twitch_api2::helix::users::GetUsersFollowsRequest;
use twitch_api2::helix::users::{GetUsersRequest, UsersFollow};

const FIRST: usize = 100;

async fn get_token() -> Result<UserToken, Error>{
    // load from dotenv file
    dotenv::dotenv().ok();

    let _client_id = twitch_oauth2::ClientId::new(std::env::var("TWITCH_CLIENT_ID").unwrap());

    let token = UserToken::from_existing(
        surf_http_client,
        AccessToken::new(
            std::env::var("TWITCH_ACCESS_TOKEN").expect("TWITCH_ACCESS_TOKEN not defined."),
        ),
        RefreshToken::new(
            std::env::var("TWITCH_REFRESH_TOKEN").expect("TWITCH_REFRESH_TOKEN not defined."),
        ),
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
    let followers: Vec<UsersFollow> = response.data.into();

    let mut followed_accounts = Vec::new();
    followers
        .iter()
        .for_each(|f| followed_accounts.push(f.to_id.clone()));

    let req = GetStreamsRequest::builder()
        .user_id(followed_accounts[1..FIRST].to_vec())
        .first(FIRST)
        .build();

    let results: Vec<Stream> = client_helix.req_get(req, &token).await?.data;

    for stream in results {
        println!(
            "{:width$}|{viewers}|{title}",
            stream.user_name,
            viewers = stream.viewer_count,
            title = stream.title,
            width = 16
        );
    }

    Ok(())
}
