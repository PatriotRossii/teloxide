use serde::{Deserialize, Serialize};

use super::{EncryptedCredentials, EncryptedPassportElement};

/// Contains information about Telegram Passport data shared with the bot by the
/// user.
///
/// [The official docs](https://core.telegram.org/bots/api#passportdata).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport
    /// elements that was shared with the bot.
    pub data: Vec<EncryptedPassportElement>,

    /// Encrypted credentials required to decrypt the data.
    pub credentials: EncryptedCredentials,
}

impl PassportData {
    pub fn new<E>(data: E, credentials: EncryptedCredentials) -> Self
    where
        E: Into<Vec<EncryptedPassportElement>>,
    {
        Self { data: data.into(), credentials }
    }

    pub fn data<E>(mut self, val: E) -> Self
    where
        E: Into<Vec<EncryptedPassportElement>>,
    {
        self.data = val.into();
        self
    }

    pub fn credentials(mut self, val: EncryptedCredentials) -> Self {
        self.credentials = val;
        self
    }
}
