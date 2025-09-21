use promkit::core::{
    crossterm::style::{ContentStyle, Stylize},
    grapheme::StyledGraphemes,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoText {
    pub id: String,
    pub desc: String,
    pub link: Option<String>,
    pub complete: Option<String>,
}

impl ToString for TodoText {
    fn to_string(&self) -> String {
        format!(
            "#{} {}{}{}",
            self.id,
            self.desc,
            self.link
                .clone()
                .map_or_else(String::new, |s| format!("({s})")),
            self.complete.clone().unwrap_or_else(String::new)
        )
    }
}

#[derive(Clone)]
pub struct TodoStyle {
    pub id: ContentStyle,
    pub desc: ContentStyle,
    pub link: ContentStyle,
    pub complete: ContentStyle,
}

impl TodoStyle {
    pub fn active_defautl_style() -> Self {
        TodoStyle {
            id: ContentStyle::new().dim().bold().italic(),
            desc: ContentStyle::new().blue(),
            link: ContentStyle::new().cyan().italic(),
            complete: ContentStyle::default(),
        }
    }
    pub fn inactive_defautl_style() -> Self {
        TodoStyle {
            id: ContentStyle::new().dim().italic(),
            desc: ContentStyle::default(),
            link: ContentStyle::new().dim().cyan().italic(),
            complete: ContentStyle::default(),
        }
    }

    pub fn format_items(
        &self,
        todo: TodoText,
        is_active: bool,
    ) -> StyledGraphemes {
        let mut styled = Vec::new();
        styled.push(StyledGraphemes::from_str(
            if todo.complete.is_some() {
                "✔ "
            } else {
                "  "
            },
            self.id,
        ));
        styled.push(StyledGraphemes::from_str(
            format!("#{} ", todo.id,),
            self.id,
        ));
        styled.push(StyledGraphemes::from_str(
            format!("{}", todo.desc,),
            self.desc,
        ));

        if is_active && todo.link.is_some() {
            styled.push(StyledGraphemes::from_str(
                format!(" ({})", todo.link.unwrap()),
                self.link,
            ));
        }

        StyledGraphemes::from_iter(styled)
    }
}
