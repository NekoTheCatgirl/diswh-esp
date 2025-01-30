use super::{edit::EditMessagePacket, embed::Embed};

#[derive(Clone)]
pub struct EditMessageBuilder {
    message: EditMessagePacket,
}

impl EditMessageBuilder {
    /// Constructs a new edit packet.
    /// 
    /// `content` may be a empty string if you only want to use embeds
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            message: EditMessagePacket {
                content: content.into(),
                embeds: Vec::new(),
            },
        }
    }

    /// Adds an embed to the edit packet.
    /// 
    /// # Note
    /// A max of 10 embeds may be put on a single message. Gateway error will occur when embed count exceeds 10
    /// 
    /// Use the provided [super::EmbedBuilder] to aid you in constructing the embed
    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.message.embeds.push(embed);
        self
    }

    /// Decomposes the Edit builder into its base packet.
    /// 
    /// # Warning
    /// This action is destructive, use only at the end.
    pub fn build(self) -> EditMessagePacket {
        self.message
    }
}
