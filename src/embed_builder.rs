use super::embed::*;

/// A helper struct to make it easy to construct a complex [super::Embed] using a chain style.
#[derive(Clone)]
pub struct EmbedBuilder {
    embed: Embed,
}

impl EmbedBuilder {
    /// Initialize a blank embed builder.
    pub fn new() -> Self {
        Self {
            embed: Embed::new(),
        }
    }

    /// Sets a title to your embed
    /// 
    /// # Panics
    /// Will panic when the provided `title` is not able to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_title("Hello, world!")
    ///     .build();
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.embed.title = Some(title.into());
        self
    }

    /// Sets a description to your embed
    /// 
    /// # Panics
    /// Will panic when the provided `description` is not able to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_description("Hello, world!");
    ///     .build();
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.embed.description = Some(description.into());
        self
    }

    /// Adds a url to your embed
    /// 
    /// This url will be masked using your [Embed::title], to use you must set a title!
    /// 
    /// # Panics
    /// Will panic if the provided `url` is not able to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_title("Hello, world!")
    ///     .with_url("https://as2.ftcdn.net/v2/jpg/09/62/15/19/1000_F_962151962_LIevlntwrKYOjOIbkXfAMETzu5wgkONs.jpg")
    ///     .build();
    /// ```
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.embed.url = Some(url.into());
        self
    }

    /// Adds footer text to your embed
    /// 
    /// # Panics
    /// Will panic if the provided `text` is not able to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_footer_text("Hello, world!")
    ///     .build();
    /// ```
    pub fn with_footer_text(mut self, text: impl Into<String>) -> Self {
        self.embed.footer.text = Some(text.into());
        self
    }

    /// Adds a footer icon url to your embed
    /// 
    /// # Panics
    /// Will panic if the provided `url` is not able to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_footer_text("Hello, world!")
    ///     .with_footer_url("https://as2.ftcdn.net/v2/jpg/09/62/15/19/1000_F_962151962_LIevlntwrKYOjOIbkXfAMETzu5wgkONs.jpg")
    ///     .build();
    /// ```
    pub fn with_footer_url(mut self, url: impl Into<String>) -> Self {
        self.embed.footer.icon_url = Some(url.into());
        self
    }

    /// Adds a image to your embed
    /// 
    /// Only 1 image is supported per embed
    /// 
    /// # Panics
    /// Will panic if the provided `url` is not able to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_image("https://as2.ftcdn.net/v2/jpg/09/62/15/19/1000_F_962151962_LIevlntwrKYOjOIbkXfAMETzu5wgkONs.jpg")
    ///     .build();
    /// ```
    pub fn with_image(mut self, url: impl Into<String>) -> Self {
        self.embed.image.url = Some(url.into());
        self
    }

    /// Sets the dimensions of your embed image
    /// 
    /// A image must be set to use this!
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_image("https://as2.ftcdn.net/v2/jpg/09/62/15/19/1000_F_962151962_LIevlntwrKYOjOIbkXfAMETzu5wgkONs.jpg")
    ///     .with_image_dims(480, 160)
    ///     .build();
    /// ```
    pub fn with_image_dims(mut self, width: i32, height: i32) -> Self {
        self.embed.image.width = Some(width);
        self.embed.image.height = Some(height);
        self
    }

    /// Adds a thumbnail to your embed
    /// 
    /// # Panics
    /// Will panic if the provided `url` is not able to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_thumbnail("https://as2.ftcdn.net/v2/jpg/09/62/15/19/1000_F_962151962_LIevlntwrKYOjOIbkXfAMETzu5wgkONs.jpg")
    ///     .build();
    /// ```
    pub fn with_thumbnail(mut self, url: impl Into<String>) -> Self {
        self.embed.thumbnail.url = Some(url.into());
        self
    }

    /// Sets the dimensions of your embed thumbnail
    /// 
    /// A thumbnail must be set to use this!
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_thumbnail("https://as2.ftcdn.net/v2/jpg/09/62/15/19/1000_F_962151962_LIevlntwrKYOjOIbkXfAMETzu5wgkONs.jpg")
    ///     .with_thumbnail_dims(480, 160)
    ///     .build();
    /// ```
    pub fn with_thumbnail_dims(mut self, width: i32, height: i32) -> Self {
        self.embed.thumbnail.width = Some(width);
        self.embed.thumbnail.height = Some(height);
        self
    }

    /// Adds a video to your embed
    /// 
    /// # Panics
    /// Will panic if the provided `url` is not able to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_video("https://www.youtube.com/watch?v=xvFZjo5PgG0")
    ///     .build();
    /// ```
    pub fn with_video(mut self, url: impl Into<String>) -> Self {
        self.embed.video.url = Some(url.into());
        self
    }

    /// Sets the dimensions of your embed video
    /// 
    /// A video must be set to use this!
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_thumbnail("https://www.youtube.com/watch?v=xvFZjo5PgG0")
    ///     .with_thumbnail_dims(480, 160)
    ///     .build();
    /// ```
    pub fn with_video_dims(mut self, width: i32, height: i32) -> Self {
        self.embed.video.width = Some(width);
        self.embed.video.height = Some(height);
        self
    }

    /// This is a mystery, but it is referenced as part of the embed class within the discord developer documentation. So its included just in case.
    pub fn with_provider(mut self, name: impl Into<String>, url: impl Into<String>) -> Self {
        self.embed.provider.name = Some(name.into());
        self.embed.provider.url = Some(url.into());
        self
    }

    /// Sets the author of the embed
    /// 
    /// # Panics
    /// Will panic if the provided `name` is not able to be converted into a [String]
    /// 
    /// # Optionals
    /// `url` can be used to make the author name link somewhere
    /// `icon_url` can be used to show the author's avatar, or a random image
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_author("John Doe", None, None)
    ///     .build();
    /// ```
    pub fn with_author(
        mut self,
        name: impl Into<String>,
        url: Option<&str>,
        icon_url: Option<&str>,
    ) -> Self {
        self.embed.author.name = Some(name.into());
        if let Some(url) = url {
            self.embed.author.url = Some(url.to_string());
        }
        if let Some(icon_url) = icon_url {
            self.embed.author.icon_url = Some(icon_url.to_string());
        }
        self
    }

    /// Sets the embed color
    /// 
    /// Either provide a:
    /// - numerical code, 
    /// - use one of the presets in the [super::Color] struct,
    /// - or use the [super::Color::from_hex] function to convert a hex code into decimal
    /// 
    /// Note, the embed must have some form of content for this to work.
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::{EmbedBuilder, Color};
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .with_title("A green embed")
    ///     .with_color(Color::GREEN)
    ///     .build();
    /// ```
    pub fn with_color(mut self, color: i32) -> Self {
        self.embed.color = color;
        self
    }

    /// Add a field to your embed
    /// 
    /// A max of 25 fields can be added to 1 embed
    /// 
    /// `inline` can be used to place 2 fields next to eachother
    /// 
    /// # Panics
    /// Will panic if either `name` or `value` are unable to be converted into a [String]
    /// 
    /// # Example
    /// ```no_run
    /// use diswh_esp::EmbedBuilder;
    /// 
    /// let embed = EmbedBuilder::new()
    ///     .add_field("Hello, world!", "Lorem ipsum", false)
    ///     .build();
    /// ```
    pub fn add_field(mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        self.embed.fields.push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    /// Decompose the inner embed into a owned object
    /// 
    /// # Warning
    /// This action is destructive, the embed builder will no longer exist after performing this action.
    pub fn build(self) -> Embed {
        self.embed
    }
}
