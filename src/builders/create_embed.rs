use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct SendableEmbed {
    #[serde(rename = "type")]
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<String>,
}

pub struct EmbedBuilder(SendableEmbed);

impl EmbedBuilder {
    pub fn new() -> Self {
        Self(SendableEmbed {
            kind: "Website".to_string(),
            ..Default::default()
        })
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.0.title = Some(title.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.0.description = Some(desc.into());
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.0.colour = Some(color.into());
        self
    }

    pub fn icon(mut self, url: impl Into<String>) -> Self {
        self.0.icon_url = Some(url.into());
        self
    }

    pub fn build(self) -> SendableEmbed {
        self.0
    }
}

impl Default for EmbedBuilder {
    fn default() -> Self {
        Self::new()
    }
}