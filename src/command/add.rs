use crate::{
    command::RunMut,
    context::Context,
    model::Todo,
    ui::form::AddUI,
    util::{SyncBlock, get_list},
};
use anyhow::Result;
use chrono::Local;
use clap::Args;
use log::{debug, info, trace};

#[derive(Debug, Args)]
pub struct AddOptions {
    /// Add todo into the child of PARENT_TODO_ID
    #[arg(short, long, value_name = "PARENT_TODO_ID")]
    into: Option<u32>,
    #[command(flatten)]
    todo_from_args: Option<AddArgs>,
    /// Add todo in specified list
    #[arg(short, long, value_name = "LIST_NAME")]
    list: Option<String>,
}

#[derive(Debug, Args)]
struct AddArgs {
    /// (required) Todo description. It is recommended to wrap it in quotes
    #[arg(
        long,
        required = false,
        value_name = "TODO_NAME",
        value_parser=clap::builder::NonEmptyStringValueParser::new(),
        help_heading = "Todo"
    )]
    desc: String,
    /// Link that can be opened by xdg-open
    #[arg(long, help_heading = "Todo")]
    link: Option<String>,
}

impl RunMut for AddOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        trace!(target: "add", "{self:#?}");
        let (desc, link) = match self.todo_from_args {
            Some(AddArgs { desc, link }) => {
                debug!(target: "add", "desc from arg: {desc}");
                debug!(target: "add", "link from arg: {link:?}");
                (desc, link)
            }
            None => {
                let rt = SyncBlock::new()?;
                let (desc, link) = rt.block_on(AddUI::run())?;
                debug!(target: "add", "desc from ui: {desc}");
                debug!(target: "add", "link from ui: {link:?}");
                (desc, link)
            }
        };

        let list = get_list(self.list, ctx);
        debug!(target: "add", "list [{list}]");

        let create_at = Local::now().date_naive();
        info!(target: "add", "todo create at: {create_at:?}");

        let todo_id = ctx.store.generate_id(&list)?;
        info!(target: "add", "todo id: {todo_id:?}");

        if let Some(pa_id) = &self.into {
            let pa_todo = ctx.store.todo_by_id_mut(&list, &pa_id)?;
            pa_todo.children.get_or_insert_with(Vec::new).push(todo_id);
        }

        let todo = Todo::new(
            desc.clone(),
            link.clone(),
            None,
            self.into,
            create_at,
            None,
        );
        debug!(target: "add", "todo instance: {todo:#?}");

        let todo = ctx.store.todos_mut(&list)?.insert(todo_id, todo);

        println!("{:#?}", todo);

        ctx.store.write(&ctx.store_path)
    }
}
