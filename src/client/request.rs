use crate::error::Result;
use oauth2::AccessToken;
use reqwest::header::{HeaderMap as Headers, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, Method, RequestBuilder};
use url::Url;

#[derive(Debug)]
pub struct Request {
    pub body: Option<Vec<u8>>,
    pub method: Method,
    pub headers: Option<Headers>,
    pub url: Url,
    // pub params: Option<Vec<&'static str, [String]>>,
}

impl Request {
    pub fn build(self, client: &Client, access_token: AccessToken) -> Result<RequestBuilder> {
        // if let Some(params) = self.params {
        //     for (param, value) in params {
        //         write!(self.url.clone(), "&{param}={value}")
        //     }
        // }

        let mut headers = self.headers.unwrap_or_default();
        let mut builder = client.request(self.method, self.url);

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", access_token.secret().as_str()).as_str())
                .unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(builder.headers(headers))
    }
}
