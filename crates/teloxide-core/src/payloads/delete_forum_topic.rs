//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{Recipient, True};

impl_payload! {
    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the _can\_delete\_messages_ administrator rights. Returns True on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub DeleteForumTopic (DeleteForumTopicSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Unique identifier for the target message thread of the forum topic
            pub message_thread_id: i32,
        }
    }
}
