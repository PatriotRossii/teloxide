use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to a photo stored on the Telegram servers.
///
/// By default, this photo will be sent by the user with an optional caption.
/// Alternatively, you can use `input_message_content` to send a message with
/// the specified content instead of the photo.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedphoto).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InlineQueryResultCachedPhoto {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier of the photo.
    pub photo_file_id: String,

    /// Title for the result.
    pub title: Option<String>,

    /// Short description of the result.
    pub description: Option<String>,

    /// Caption of the photo to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the photo.
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedPhoto {
    pub fn new<S1, S2>(id: S1, photo_file_id: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            photo_file_id: photo_file_id.into(),
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn photo_file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.photo_file_id = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(val.into());
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = Some(val.into());
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode<S>(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}
