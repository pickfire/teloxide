//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{Recipient, True};

impl_payload! {
    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the _can\_manage\_topics_ administrator rights, unless it is the creator of the topic. Returns True on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub ReopenForumTopic (ReopenForumTopicSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Unique identifier for the target message thread of the forum topic
            pub message_thread_id: i32,
        }
    }
}
