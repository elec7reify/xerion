use serde::{Deserialize, Serialize};

/// Indicates whether the content being withheld is the &#x60;tweet&#x60; or a &#x60;user&#x60;.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WithheldScope {
    Tweet,
    User,
}

/// Indicates withholding details for [withheld content](https://help.x.com/en/rules-and-policies/post-withheld-by-country).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Withheld {
    /// Indicates if the content is being withheld for on the basis of copyright infringement.
    pub copyright: bool,

    /// Provides a list of countries where this user is not available.
    pub country_codes: Vec<String>,

    /// Indicates whether the content being withheld is a Tweet or a user (this API will return `user`).
    pub scope: Option<WithheldScope>,
}
