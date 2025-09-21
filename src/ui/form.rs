use crate::util::IntoOption;
use anyhow::Result;
use promkit::{
    Prompt,
    core::crossterm::style::{ContentStyle, Stylize},
    preset::readline::Readline,
};

trait InitialTextEx {
    fn initial_text<T: AsRef<str>>(self, initial_text: T) -> Self;
}

impl InitialTextEx for Readline {
    fn initial_text<T: AsRef<str>>(mut self, initial_text: T) -> Self {
        self.readline.texteditor.replace(initial_text.as_ref());
        self
    }
}

pub struct AddUI;

impl AddUI {
    pub async fn run() -> Result<(String, Option<String>)> {
        let desc = readline_render("What to do?", "> ", None, false).await?;
        let link_raw =
            readline_render("Related link", "> ", None, true).await?;

        let link = link_raw.into_option();

        Ok((desc, link))
    }
}

pub struct ModifyUI;

impl ModifyUI {
    pub async fn run(
        old_desc: &String,
        old_link: &Option<String>,
    ) -> Result<(Option<String>, Option<String>)> {
        let new_desc_raw = readline_render(
            old_desc.as_str(),
            "Change to > ",
            Some(old_desc.as_str()),
            false,
        )
        .await?;
        let new_desc = new_desc_raw.into_option();

        let Some(old_link_raw) = old_link else {
            let new_link_raw =
                readline_render("Add a related link", ">", Some(""), true)
                    .await?;
            let new_link = new_link_raw.into_option();

            return Ok((new_desc, new_link));
        };

        let new_link_raw = readline_render(
            old_link_raw.as_str(),
            "Change to > ",
            Some(old_link_raw.as_str()),
            true,
        )
        .await?;
        let new_link = new_link_raw.into_option();

        Ok((new_desc, new_link))
    }
}

async fn readline_render<T: AsRef<str>>(
    title: T,
    prefix: T,
    placeholder: Option<T>,
    empty: bool,
) -> Result<String> {
    let mut rl = Readline::default()
        .text_editor_lines(10)
        .title(title)
        .title_style(ContentStyle::new().blue())
        .prefix(prefix)
        .prefix_style(ContentStyle::new().dark_grey())
        .active_char_style(ContentStyle::new().black().on_blue());
    if let Some(placeholder) = placeholder {
        rl = rl.initial_text(placeholder);
    }
    if empty {
        rl.run().await
    } else {
        rl.validator(|t| t.len() > 0, |_| String::from("Cannot be empty"))
            .run()
            .await
    }
}
