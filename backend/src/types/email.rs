use serde::{Deserialize, Serialize};
use std::fmt;

/// Type-safe wrapper for email addresses
/// Provides validation and prevents invalid email strings
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Email(String);

impl Email {
    /// Create a new Email with validation
    pub fn new(email: impl Into<String>) -> Result<Self, EmailError> {
        let email = email.into();
        Self::validate(&email)?;
        Ok(Self(email))
    }

    /// Create Email without validation (use carefully!)
    /// Useful when email is already validated (e.g., from database)
    pub fn new_unchecked(email: impl Into<String>) -> Self {
        Self(email.into())
    }

    /// Basic email validation
    fn validate(email: &str) -> Result<(), EmailError> {
        if email.is_empty() {
            return Err(EmailError::Empty);
        }

        if !email.contains('@') {
            return Err(EmailError::MissingAtSign);
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err(EmailError::InvalidFormat);
        }

        let (local, domain) = (parts[0], parts[1]);

        if local.is_empty() {
            return Err(EmailError::EmptyLocalPart);
        }

        if domain.is_empty() || !domain.contains('.') {
            return Err(EmailError::InvalidDomain);
        }

        Ok(())
    }

    /// Get email as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to String
    pub fn into_string(self) -> String {
        self.0
    }

    /// Get lowercase version of email
    pub fn to_lowercase(&self) -> Email {
        Email(self.0.to_lowercase())
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Diesel support
impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for Email
where
    DB: diesel::backend::Backend,
    String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        self.0.to_sql(out)
    }
}

impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for Email
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(Self(String::from_sql(bytes)?))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum EmailError {
    #[error("Email cannot be empty")]
    Empty,
    #[error("Email must contain @ sign")]
    MissingAtSign,
    #[error("Email has invalid format")]
    InvalidFormat,
    #[error("Email local part cannot be empty")]
    EmptyLocalPart,
    #[error("Email domain is invalid")]
    InvalidDomain,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(Email::new("user@example.com").is_ok());
        assert!(Email::new("test.user+tag@subdomain.example.com").is_ok());
    }

    #[test]
    fn test_invalid_email() {
        assert!(Email::new("").is_err());
        assert!(Email::new("no-at-sign").is_err());
        assert!(Email::new("@example.com").is_err());
        assert!(Email::new("user@").is_err());
        assert!(Email::new("user@nodot").is_err());
    }

    #[test]
    fn test_email_display() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.to_string(), "user@example.com");
    }

    #[test]
    fn test_email_lowercase() {
        let email = Email::new("User@Example.COM").unwrap();
        let lower = email.to_lowercase();
        assert_eq!(lower.as_str(), "user@example.com");
    }
}
