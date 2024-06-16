use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use reqwest::Response;
use tokio::sync::Mutex;
use url::Url;

pub struct RateLimiter {
    requests: Arc<HashMap<Url, Arc<Mutex<Ratelimit>>>>,
}

impl RateLimiter {
    // pub async fn check_if_rate_limited(&self) -> Result<Response> {
    //
    // }
}

#[derive(Debug)]
pub struct Ratelimit {
    limit: i64,
    remaining: i64,
    reset: Option<SystemTime>,
}

impl Ratelimit {
    #[inline]
    pub fn limit(&self) -> i64 {
        self.limit
    }

    #[inline]
    pub fn remaining(&self) -> i64 {
        self.remaining
    }

    #[inline]
    pub fn reset(&self) -> Option<SystemTime> {
        self.reset
    }
}
