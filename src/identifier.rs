use serde::de::{self, Deserialize, Deserializer, Error};
use std::fmt;
use std::ops::Deref;
use std::str;

/// Represents a string containing no whitespace
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Identifier(String);

impl Identifier {
    /// Creates new identifier with a given `name`.
    /// Return `Err(name)` if there is any whitespace in `name`
    pub fn new(name: String) -> Result<Self, String> {
        if name.chars().any(|ch| ch.is_whitespace()) {
            return Err(name);
        }
        Ok(Identifier(name))
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        if name.chars().any(|ch| ch.is_whitespace()) {
            Err(Error::invalid_value(
                de::Unexpected::Str(&name),
                &"a string without whitespace",
            ))
        } else {
            Ok(Identifier(name))
        }
    }
}

impl Deref for Identifier {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl str::FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Identifier::new(s.into()).map_err(|name| format!("invalid identifier: {}", name))
    }
}