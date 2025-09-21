use crate::{
    command::RunMut,
    context::Context,
    ui::form::ModifyUI,
    util::{SyncBlock, get_list},
};
use anyhow::Result;
use clap::Args;
use log::{debug, trace};

#[derive(Debug, Args)]
pub struct ModifyOptions {
    #[arg(required = true)]
    todo_id: u32,
    #[command(flatten)]
    todo_from_args: Option<ModifyArgs>,
    /// Modify todo in specified list
    #[arg(short, long, value_name = "LIST_NAME")]
    list: Option<String>,
}

#[derive(Debug, Args)]
struct ModifyArgs {
    /// Todo description. It is recommended to wrap it in quotes
    #[arg(
        long,
        value_name = "TODO_NAME",
        value_parser=clap::builder::NonEmptyStringValueParser::new(),
        help_heading = "Todo"
    )]
    desc: Option<String>,
    /// Link that can be opened by xdg-open
    #[arg(long, help_heading = "Todo")]
    link: Option<String>,
}

impl RunMut for ModifyOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        trace!(target: "modify", "{self:#?}");

        let list = get_list(self.list, ctx);
        debug!(target: "modify", "list [{list}]");

        debug!(target: "modify", "todo [{:?}]", self.todo_id);
        let todo = ctx.store.todo_by_id_mut(&list, &self.todo_id)?;

        let (desc, link) = match self.todo_from_args {
            Some(ModifyArgs { desc, link }) => {
                debug!(target: "modify", "desc from arg: {desc:?}");
                debug!(target: "modify", "link from arg: {link:?}");

                (desc, link)
            }
            None => {
                let rt = SyncBlock::new()?;
                rt.block_on(ModifyUI::run(&todo.desc, &todo.link))?
            }
        };

        if let Some(desc) = desc {
            debug!(target: "modify", "change desc to: {desc}");
            todo.desc = desc;
        }
        if link.is_some() {
            debug!(target: "modify", "change link to: {link:?}");
            todo.link = link;
        }

        println!("{}", todo);

        ctx.store.write(&ctx.store_path)
    }
}
