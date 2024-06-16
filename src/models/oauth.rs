use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;

/// OAuth2 Scopes.
///
/// [Official Documentation](https://developer.x.com/en/docs/authentication/oauth-2-0/authorization-code).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Scope {
    /// All the Tweets you can view, including Tweets from protected accounts.
    #[serde(rename = "tweet.read")]
    TweetRead,

    /// Tweet and Retweet for you.
    #[serde(rename = "tweet.write")]
    TweetWrite,

    /// Hide and unhide replies to your Tweets.
    #[serde(rename = "tweet.moderate.write")]
    TweetModerateWrite,

    /// Any account you can view, including protected accounts.
    #[serde(rename = "users.read")]
    UsersRead,

    /// People who follow you and people who you follow.
    #[serde(rename = "follows.read")]
    FollowsRead,

    /// Follow and unfollow people for you.
    #[serde(rename = "follows.write")]
    FollowsWrite,

    /// Stay connected to your account until you revoke access.
    #[serde(rename = "offline.access")]
    OfflineAccess,

    /// All the Spaces you can view.
    #[serde(rename = "space.read")]
    SpaceRead,

    /// Accounts you’ve muted.
    #[serde(rename = "mute.read")]
    MuteRead,

    /// Mute and unmute accounts for you.
    #[serde(rename = "mute.write")]
    MuteWrite,

    /// Tweets you’ve liked and likes you can view.
    #[serde(rename = "like.read")]
    LikeRead,

    /// Like and un-like Tweets for you.
    #[serde(rename = "like.write")]
    LikeWrite,

    /// Lists, list members, and list followers of lists you’ve created or are a member of
    /// including private lists.
    #[serde(rename = "list.read")]
    ListRead,

    /// Create and manage Lists for you.
    #[serde(rename = "list.write")]
    ListWrite,

    /// Accounts you’ve blocked.
    #[serde(rename = "block.read")]
    BlockRead,

    /// Block and unblock accounts for you.
    #[serde(rename = "block.write")]
    BlockWrite,

    /// Get Bookmarked Tweets from an authenticated user.
    #[serde(rename = "bookmark.read")]
    BookmarkRead,

    /// Bookmark and remove Bookmarks from Tweets.
    #[serde(rename = "bookmark.write")]
    BookmarkWrite,
}

impl From<Scope> for oauth2::Scope {
    fn from(scope: Scope) -> Self {
        oauth2::Scope::new(scope.to_string())
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.serialize(f)
    }
}
