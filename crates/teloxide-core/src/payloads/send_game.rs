//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{ChatId, Message, ReplyMarkup, ReplyParameters, ThreadId};

impl_payload! {
    /// Use this method to send a game. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendGame (SendGameSetters) => Message {
        required {
            /// Unique identifier for the target chat
            pub chat_id: ChatId [into],
            /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
            pub game_short_name: String [into],
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the message will be sent
            pub business_connection_id: String [into],
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// Description of the message to reply to
            pub reply_parameters: ReplyParameters,
            /// A JSON-serialized object for an [inline keyboard]. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game. Not supported for messages sent on behalf of a business account.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
