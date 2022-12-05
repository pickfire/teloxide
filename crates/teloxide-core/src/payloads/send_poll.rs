//! Generated by `codegen_payloads`, do not edit by hand.

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::types::{
    Message, MessageEntity, MessageId, ParseMode, PollType, Recipient, ReplyMarkup,
};

impl_payload! {
    /// Use this method to send phone contacts. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendPoll (SendPollSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Poll question, 1-300 characters
            pub question: String [into],
            /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
            pub options: Vec<String> [collect],
        }
        optional {
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: i32,
            /// True, if the poll needs to be anonymous, defaults to True
            pub is_anonymous: bool,
            /// Poll type, “quiz” or “regular”, defaults to “regular”
            #[serde(rename = "type")]
            pub type_: PollType,
            /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
            pub allows_multiple_answers: bool,
            /// 0-based identifier of the correct answer option, required for polls in quiz mode
            pub correct_option_id: u8,
            /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
            pub explanation: String [into],
            /// Mode for parsing entities in the message text. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub explanation_parse_mode: ParseMode,
            /// List of special entities that appear in the poll explanation, which can be specified instead of _parse\_mode_
            pub explanation_entities: Vec<MessageEntity> [collect],
            /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
            pub open_period: u16,
            /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period.
            #[serde(with = "crate::types::serde_opt_date_from_unix_timestamp")]
            pub close_date: DateTime<Utc> [into],
            /// Pass True, if the poll needs to be immediately closed. This can be useful for poll preview.
            pub is_closed: bool,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// If the message is a reply, ID of the original message
            #[serde(serialize_with = "crate::types::serialize_reply_to_message_id")]
            pub reply_to_message_id: MessageId,
            /// Pass _True_, if the message should be sent even if the specified replied-to message is not found
            pub allow_sending_without_reply: bool,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
