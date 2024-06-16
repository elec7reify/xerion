use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use std::num::NonZeroU64;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(pub NonZeroU64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(NonZeroU64);

impl Serialize for UserId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = Deserialize::deserialize(deserializer)?;
        u64::from_str(&s)
            .ok()
            .and_then(NonZeroU64::new)
            .map(UserId)
            .ok_or_else(|| DeError::custom("Invalid ID: must be non-zero"))
    }
}

#[cfg(test)]
mod tests {
    use crate::models::id::UserId;
    use serde::{Deserialize, Serialize};
    use std::num::NonZeroU64;

    #[test]
    fn id() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct S {
            user_id: UserId,
        }

        let s = S {
            user_id: UserId(NonZeroU64::new(10_7654_3210_0123_4567_8).unwrap()),
        };
        assert_eq!(s.user_id.0, "1076543210012345678".parse().unwrap())
    }
}
