use serde::{Deserialize, Serialize};

use crate::types::{InputFile, ParseMode};

/// This object represents the content of a media message to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedia).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputMedia {
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
}

/// Represents a photo to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaphoto).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputMediaPhoto {
    /// File to send.
    pub media: InputFile,

    /// Caption of the photo to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,
}

impl InputMediaPhoto {
    pub fn new(media: InputFile) -> Self {
        Self { media, caption: None, parse_mode: None }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }
}

/// Represents a video to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediavideo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputMediaVideo {
    // File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumb: Option<InputFile>,

    /// Caption of the video to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// Video width.
    pub width: Option<u16>,

    /// Video height.
    pub height: Option<u16>,

    /// Video duration.
    pub duration: Option<u16>,

    /// Pass `true`, if the uploaded video is suitable for streaming.
    pub supports_streaming: Option<bool>,
}

impl InputMediaVideo {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn width(mut self, val: u16) -> Self {
        self.width = Some(val);
        self
    }

    pub fn height(mut self, val: u16) -> Self {
        self.height = Some(val);
        self
    }

    pub fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }

    pub fn supports_streaming(mut self, val: bool) -> Self {
        self.supports_streaming = Some(val);
        self
    }
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without
/// sound) to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaanimation).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputMediaAnimation {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumb: Option<InputFile>,

    /// Caption of the animation to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// Animation width.
    pub width: Option<u16>,

    /// Animation height.
    pub height: Option<u16>,

    /// Animation duration.
    pub duration: Option<u16>,
}

impl InputMediaAnimation {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn width(mut self, val: u16) -> Self {
        self.width = Some(val);
        self
    }

    pub fn height(mut self, val: u16) -> Self {
        self.height = Some(val);
        self
    }

    pub fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }
}

/// Represents an audio file to be treated as music to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaaudio).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputMediaAudio {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumb: Option<InputFile>,

    /// Caption of the audio to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// Duration of the audio in seconds.
    pub duration: Option<u16>,

    /// Performer of the audio.
    pub performer: Option<String>,

    /// Title of the audio.
    pub title: Option<String>,
}

impl InputMediaAudio {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            performer: None,
            title: None,
            duration: None,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }

    pub fn performer<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.performer = Some(val.into());
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(val.into());
        self
    }
}

/// Represents a general file to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediadocument).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputMediaDocument {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumb: Option<InputFile>,

    /// Caption of the document to be sent, 0-1024 charactersю
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,
}

impl InputMediaDocument {
    pub fn new(media: InputFile) -> Self {
        Self { media, thumb: None, caption: None, parse_mode: None }
    }

    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }
}

impl InputMedia {
    pub fn media(&self) -> &InputFile {
        match self {
            InputMedia::Photo(InputMediaPhoto { media, .. })
            | InputMedia::Document(InputMediaDocument { media, .. })
            | InputMedia::Audio(InputMediaAudio { media, .. })
            | InputMedia::Animation(InputMediaAnimation { media, .. })
            | InputMedia::Video(InputMediaVideo { media, .. }) => media,
        }
    }
}

impl From<InputMedia> for InputFile {
    fn from(media: InputMedia) -> InputFile {
        match media {
            InputMedia::Photo(InputMediaPhoto { media, .. })
            | InputMedia::Document(InputMediaDocument { media, .. })
            | InputMedia::Audio(InputMediaAudio { media, .. })
            | InputMedia::Animation(InputMediaAnimation { media, .. })
            | InputMedia::Video(InputMediaVideo { media, .. }) => media,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn photo_serialize() {
        let expected_json = r#"{"type":"photo","media":"123456"}"#;
        let photo = InputMedia::Photo(InputMediaPhoto {
            media: InputFile::FileId(String::from("123456")),
            caption: None,
            parse_mode: None,
        });

        let actual_json = serde_json::to_string(&photo).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn video_serialize() {
        let expected_json = r#"{"type":"video","media":"123456"}"#;
        let video = InputMedia::Video(InputMediaVideo {
            media: InputFile::FileId(String::from("123456")),
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn animation_serialize() {
        let expected_json = r#"{"type":"animation","media":"123456"}"#;
        let video = InputMedia::Animation(InputMediaAnimation {
            media: InputFile::FileId(String::from("123456")),
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn audio_serialize() {
        let expected_json = r#"{"type":"audio","media":"123456"}"#;
        let video = InputMedia::Audio(InputMediaAudio {
            media: InputFile::FileId(String::from("123456")),
            thumb: None,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn document_serialize() {
        let expected_json = r#"{"type":"document","media":"123456"}"#;
        let video = InputMedia::Document(InputMediaDocument {
            media: InputFile::FileId(String::from("123456")),
            thumb: None,
            caption: None,
            parse_mode: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
