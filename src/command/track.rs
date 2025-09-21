use crate::{command::Run, context::Context, util::get_list};
use anyhow::Result;
use clap::Args;
use log::{debug, trace};

#[derive(Debug, Args)]
pub struct TrackOptions {
    todo_id: u32,
    /// List todos in specified list
    #[arg(short, long, value_name = "LIST_NAME")]
    list: Option<String>,
}

impl Run for TrackOptions {
    fn run(self, ctx: &Context) -> Result<()> {
        trace!(target: "track", "{self:#?}");

        let list = get_list(self.list, ctx);
        debug!(target: "track", "list [{list}]");
        debug!(target: "track", "todo [{}]", self.todo_id);

        let todo = ctx.store.todo_by_id(&list, &self.todo_id)?;
        println!("{}", todo);

        Ok(())
    }
}
