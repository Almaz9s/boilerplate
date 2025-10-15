use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Type-safe wrapper for User ID
/// Prevents accidental mixing of different ID types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(Uuid);

impl UserId {
    /// Create a new random UserId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create UserId from existing UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Parse UserId from string
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get inner UUID value
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to UUID
    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl std::str::FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

// Diesel support
impl<DB> diesel::serialize::ToSql<diesel::sql_types::Uuid, DB> for UserId
where
    DB: diesel::backend::Backend,
    Uuid: diesel::serialize::ToSql<diesel::sql_types::Uuid, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        self.0.to_sql(out)
    }
}

impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Uuid, DB> for UserId
where
    DB: diesel::backend::Backend,
    Uuid: diesel::deserialize::FromSql<diesel::sql_types::Uuid, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(Self(Uuid::from_sql(bytes)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_creation() {
        let id = UserId::new();
        assert_ne!(id.as_uuid(), &Uuid::nil());
    }

    #[test]
    fn test_user_id_parse() {
        let uuid = Uuid::new_v4();
        let id = UserId::parse(&uuid.to_string()).unwrap();
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_user_id_display() {
        let uuid = Uuid::new_v4();
        let id = UserId::from_uuid(uuid);
        assert_eq!(id.to_string(), uuid.to_string());
    }
}
