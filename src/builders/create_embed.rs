use crate::model::message::Embed;

pub struct EmbedBuilder(Embed);
impl EmbedBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(self) -> Embed {
        self.0
    }
    pub fn set_icon(mut self, icon_url: impl Into<String>) -> Self {
        self.0.icon_url = Some(icon_url.into());
        self
    }
    pub fn set_url(mut self, url: impl Into<String>) -> Self {
        self.0.url = Some(url.into());
        self
    }
    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.0.title = Some(title.into());
        self
    }
    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.0.description = Some(description.into());
        self
    }
    pub fn set_media(mut self, media: impl Into<String>) -> Self {
        self.0.media = Some(media.into());
        self
    }
    pub fn set_colour(mut self, colour: impl Into<String>) -> Self {
        self.0.colour = Some(colour.into());
        self
    }
}

impl Default for EmbedBuilder {
    fn default() -> EmbedBuilder {
        Self(Embed {
            icon_url: None,
            url: None,
            title: None,
            description: None,
            media: None,
            colour: None,
        })
    }
}
impl From<Embed> for EmbedBuilder {
    fn from(embed: Embed) -> Self {
        Self(Embed {
            icon_url: embed.icon_url,
            url: embed.url,
            title: embed.title,
            description: embed.description,
            media: embed.media,
            colour: embed.colour,
        })
    }
}
