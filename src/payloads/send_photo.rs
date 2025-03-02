//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{
    InputFile, Message, MessageEntity, MessageId, ParseMode, Recipient, ReplyMarkup,
};

impl_payload! {
    @[multipart = photo]
    /// Use this method to send photos. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, Clone, Serialize)]
    pub SendPhoto (SendPhotoSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub photo: InputFile,
        }
        optional {
            /// Photo caption (may also be used when resending photos by _file\_id_), 0-1024 characters after entities parsing
            pub caption: String [into],
            /// Mode for parsing entities in the photo caption. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// List of special entities that appear in the photo caption, which can be specified instead of _parse\_mode_
            pub caption_entities: Vec<MessageEntity> [collect],
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
            /// Pass True if the photo needs to be covered with a spoiler animation
            pub has_spoiler: bool,
        }
    }
}
