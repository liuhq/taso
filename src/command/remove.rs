use crate::{
    command::RunMut,
    context::Context,
    util::{check_key, get_list},
};
use anyhow::Result;
use clap::Args;
use log::{debug, trace, warn};

#[derive(Debug, Args)]
pub struct RemoveOptions {
    #[arg(required = true, value_delimiter = ',')]
    todo_id: Vec<u32>,
    /// Remove exist todo(s) and its (or their) children recursively
    #[arg(short, long)]
    recursive: bool,
    /// Remove todos from specified list
    #[arg(short, long, value_name = "LIST_NAME")]
    list: Option<String>,
}

impl RunMut for RemoveOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        trace!(target: "remove", "{self:#?}");

        let list = get_list(self.list, ctx);
        debug!(target: "remove", "list [{list}]");
        debug!(target: "remove", "to be removed: todo [{:?}]", self.todo_id);

        let todos_mut = ctx.store.todos_mut(&list)?;
        let mut removed_todos = Vec::new();

        for id in self.todo_id.iter() {
            check_key(todos_mut, id)?;
            let Some(removed) = todos_mut.remove(id) else {
                warn!(target: "remove", "[{id}] can't remove");
                continue;
            };
            removed_todos.push(removed);
        }

        println!("{:#?}", removed_todos);

        ctx.store.write(&ctx.store_path)
    }
}
