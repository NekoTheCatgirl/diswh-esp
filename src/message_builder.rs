use super::{embed::Embed, message::MessagePacket};

#[derive(Clone)]
pub struct MessageBuilder {
    message: MessagePacket,
}

impl MessageBuilder {
    /// Initializes a new message builder
    /// 
    /// `content` may be left as a empty string if you do not want to have content
    /// `tts` will make the message execute text to speech on all clients who have it enabled.
    /// 
    /// # Panics
    /// Will panic if the provided `content` can not be parsed into a string.
    pub fn new(content: impl Into<String>, tts: bool) -> Self {
        Self {
            message: MessagePacket {
                content: content.into(),
                username: "".into(),
                avatar_url: "".into(),
                tts,
                embeds: Vec::new(),
            },
        }
    }

    /// Sets the message to send with a custom username.
    /// 
    /// # Panics
    /// Will panic if the provided `username` can not be parsed into a string.
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.message.username = username.into();
        self
    }

    /// Sets the message to send with a custom avatar.
    /// 
    /// # Panics
    /// Will panic if the provided `avatar_url` can not be parsed into a string.
    pub fn with_avatar_url(mut self, avatar_url: impl Into<String>) -> Self {
        self.message.avatar_url = avatar_url.into();
        self
    }

    /// Adds an embed to the message packet.
    /// 
    /// # Note
    /// A max of 10 embeds may be put on a single message. Gateway error will occur when embed count exceeds 10
    /// 
    /// Use the provided [super::EmbedBuilder] to aid you in constructing the embed
    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.message.embeds.push(embed);
        self
    }

    /// Decomposes the Message builder into its base packet.
    /// 
    /// # Warning
    /// This action is destructive, use only at the end.
    pub fn build(self) -> MessagePacket {
        self.message
    }
}
