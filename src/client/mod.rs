use std::io::Read;
use futures::Stream;
use oauth2::AccessToken;
use reqwest::{IntoUrl, Method, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::error::*;
use crate::models::user::User;
use std::error::Error as Err;
use std::ops::Deref;

mod auth;
mod request;
mod error;
mod payload;
mod ratelimit;

pub use self::error::*;
pub use self::payload::*;
pub use self::request::*;

#[must_use]
pub struct Client {
    pub client: reqwest::Client,
    pub token: AccessToken,
}

impl Client {
    pub fn token(&self) -> &String {
        self.token.secret()
    }

    /// Returns information about an authorized user.
    pub async fn users_me(&self) -> Result<User>{
        self.get(Request {
            body: None,
            method: Method::GET,
            headers: None,
            url: "https://api.x.com/2/users/me?user.fields=created_at,description,entities,id,location,name,pinned_tweet_id,profile_image_url,protected,url,username,verified,verified_type,withheld,subscription_type&expansions=pinned_tweet_id".parse().unwrap(),
            params: None,
        })
        .await
    }

    pub async fn get<T: DeserializeOwned + std::fmt::Debug>(&self, req: Request) -> Result<T> {
        let response = self.request(req).await?;
        let bytes = response.bytes().await?;
        let payload: ApiPayload<T, ()> = serde_json::from_slice(&bytes)?;
        let resp = ApiResponse::new(payload);
        Ok(resp.into_data())
    }

    pub async fn request(&self, req: Request) -> Result<Response> {
        let request = req.build(&self.client, self.token.to_owned())?.build()?;
        let response = self.client.execute(request).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use oauth2::AccessToken;
    use crate::Client;

    #[tokio::test]
    async fn users() {
        let client = Client {
            client: reqwest::Client::builder().pool_max_idle_per_host(0).build().unwrap(),
            token: AccessToken::new("ejBmLVh2MDFIblZSY0tNQzRLY1lDSzZKWHFZNWQyMWlMbnNQSHF3SWhMY3JFOjE3MTQ0MDk5MzM4Nzk6MTowOmF0OjE".to_string()).to_owned(),
        };
        match client.users_me().await {
            Ok(info) => println!("User Id: {:?}, {:?}", info.pinned_tweet_id, info.verified),
            Err(e) => eprintln!("Error fetching user data: {:?}", e),
        }
    }
}
