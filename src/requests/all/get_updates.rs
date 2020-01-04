use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{AllowedUpdate, Update},
    Bot,
};

/// Use this method to receive incoming updates using long polling ([wiki]).
/// An array ([`Vec`]) of [`Update`]s is returned.
///
/// **Notes:**
/// 1. This method will not work if an outgoing webhook is set up.
/// 2. In order to avoid getting duplicate updates,
///    recalculate offset after each server response.
///
/// [wiki]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
/// [Update]: crate::types::Update
/// [Vec]: std::alloc::Vec
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetUpdates<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Identifier of the first update to be returned. Must be greater by one
    /// than the highest among the identifiers of previously received updates.
    /// By default, updates starting with the earliest unconfirmed update are
    /// returned. An update is considered confirmed as soon as [`GetUpdates`]
    /// is called with an [`offset`] higher than its [`id`]. The negative
    /// offset can be specified to retrieve updates starting from `-offset`
    /// update from the end of the updates queue. All previous updates will
    /// forgotten.
    ///
    /// [`GetUpdates`]: self::GetUpdates
    /// [`offset`]: self::GetUpdates::offset
    /// [`id`]: crate::types::Update::id
    pub offset: Option<i32>,
    /// Limits the number of updates to be retrieved.
    /// Values between `1`—`100` are accepted. Defaults to `100`.
    pub limit: Option<u8>,
    /// Timeout in seconds for long polling. Defaults to `0`,
    /// i.e. usual short polling. Should be positive, short polling should be
    /// used for testing purposes only.
    pub timeout: Option<u32>,
    /// List the types of updates you want your bot to receive.
    /// For example, specify [[`Message`], [`EditedChannelPost`],
    /// [`CallbackQuery`]] to only receive updates of these types.
    /// See [`AllowedUpdate`] for a complete list of available update types.
    ///
    /// Specify an empty list to receive all updates regardless of type
    /// (default). If not specified, the previous setting will be used.
    ///
    /// **Note:**
    /// This parameter doesn't affect updates created before the call to the
    /// [`GetUpdates`], so unwanted updates may be received for a short period
    /// of time.
    ///
    /// [`Message`]: self::AllowedUpdate::Message
    /// [`EditedChannelPost`]: self::AllowedUpdate::EditedChannelPost
    /// [`CallbackQuery`]: self::AllowedUpdate::CallbackQuery
    /// [`AllowedUpdate`]: self::AllowedUpdate
    /// [`GetUpdates`]: self::GetUpdates
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

#[async_trait::async_trait]
impl Request for GetUpdates<'_> {
    type Output = Vec<Update>;

    async fn send(&self) -> ResponseResult<Vec<Update>> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getUpdates",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> GetUpdates<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    pub fn offset(mut self, value: i32) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: u8) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn timeout(mut self, value: u32) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn allowed_updates<T>(mut self, value: T) -> Self
    where
        T: Into<Vec<AllowedUpdate>>, // TODO: into or other trait?
    {
        self.allowed_updates = Some(value.into());
        self
    }
}