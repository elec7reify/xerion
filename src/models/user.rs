use crate::models::id::UserId;
use crate::models::withheld::Withheld;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
use time::{OffsetDateTime, serde::rfc3339};

/// Enum representing the type of verification a user has undergone.
///
/// The variants include:
/// - `Blue`: Blue verified users.
/// - `Business`: Business verified users.
/// - `Government`: Government verified users.
/// - `None`: Users without any specific verification.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum VerifiedType {
    Blue,
    Business,
    Government,
    #[default]
    None,
}

/// Enum representing the type of subscription a user has.
///
/// The variants include:
/// - `Basic`: Basic subscription.
/// - `Premium`: Premium subscription.
/// - `PremiumPlus`: Premium Plus subscription.
/// - `None`: Users without a subscription.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SubscriptionType {
    Basic,
    Premium,
    PremiumPlus,
    #[default]
    None,
}

/// The X User object.
///
/// This struct represents a user within the X API. It contains various properties and methods
/// to interact with user data.
///
/// [Official Documentation](https://developer.x.com/en/docs/twitter-api/data-dictionary/object-model/user).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier of this user.
    pub id: UserId,

    /// The friendly name of this user, as shown on their profile.
    pub name: String,

    /// The Twitter handle (screen name) of this user.
    pub username: String,

    /// Creation time of this account.
    #[serde(default, with = "rfc3339")]
    pub created_at: Option<OffsetDateTime>,

    /// The ID of the User's most recent Tweet.
    pub most_recent_tweet_id: Option<String>,

    /// Indicates if this user has chosen to protect their Tweets
    /// (in other words, if this user's Tweets are private).
    pub protected: Option<bool>,

    /// Contains withholding details for withheld content.
    pub withheld: Option<Withheld>,

    /// The location specified in the user's profile, if the user provided one.
    /// As this is a freeform value, it may not indicate a valid location,
    /// but it may be fuzzily evaluated when performing searches with location queries.
    pub location: Option<String>,

    pub profile_image_url: Option<String>,

    /// The URL specified in the user's profile, if present.
    pub url: Option<String>,

    /// The text of this user's profile description (also known as bio), if the user provided one.
    pub description: Option<String>,

    /// Indicate if this user is a verified Twitter user.
    pub verified: Option<bool>,

    /// Indicates the type of verification for the Twitter account.
    #[serde(default)]
    pub verified_type: VerifiedType,

    #[serde(default)]
    pub subscription_type: SubscriptionType,

    // TODO: entities
    /// Unique identifier of this user's pinned Tweet.
    pub pinned_tweet_id: Option<String>,
}

impl From<User> for UserId {
    fn from(user: User) -> UserId {
        user.id
    }
}
