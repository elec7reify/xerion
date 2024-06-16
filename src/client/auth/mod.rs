use async_trait::async_trait;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, RevocationUrl,
    TokenUrl,
};
use reqwest::header::HeaderValue;
use serde::Serialize;
use url::Url;

use crate::error::Result;
use crate::models::Scope;
use crate::ClientError;

#[must_use]
#[async_trait]
pub trait Authorization {
    async fn header(&self) -> Result<HeaderValue, ClientError>;
}

#[derive(Debug, Clone)]
pub struct OAuth(BasicClient);

impl OAuth {
    #[must_use]
    pub fn build<S>(client_id: S, client_secret: ClientSecret, callback_url: Url) -> Self
    where
        S: Into<String>,
    {
        Self(
            BasicClient::new(
                ClientId::new(client_id.into()),
                Some(ClientSecret::from(client_secret)),
                AuthUrl::from_url("https://x.com/i/oauth2/authorize".parse().unwrap()),
                Some(TokenUrl::from_url(
                    "https://api.x.com/2/oauth2/token".parse().unwrap(),
                )),
            )
            .set_revocation_uri(RevocationUrl::from_url(
                "https://api.x.com/2/oauth2/revoke".parse().unwrap(),
            ))
            .set_redirect_uri(RedirectUrl::from_url(callback_url)),
        )
    }

    pub fn auth_url<I>(&self, challenge: PkceCodeChallenge, scopes: I) -> (Url, CsrfToken)
    where
        I: IntoIterator<Item = Scope>,
    {
        self.0
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(challenge)
            .add_scopes(scopes.into_iter().map(|m| m.into()))
            .url()
    }
}

#[derive(Debug, Clone, Serialize)]
#[must_use]
pub struct OAuthParameters {
    scopes: Vec<Scope>,
}

impl OAuthParameters {
    pub fn scopes(&self) -> &[Scope] {
        &self.scopes
    }
}
