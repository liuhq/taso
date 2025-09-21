use crate::{command::RunMut, context::Context, model::TodoMap};
use anyhow::{Result, bail};
use clap::Args;
use log::debug;

#[derive(Debug, Args)]
pub struct ListDefaultOptions {
    new_default_lists: String,
    #[arg(short, long)]
    new: bool,
}

impl RunMut for ListDefaultOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        debug!(target: "list_add", "{ctx:#?}");

        let lists_mut = ctx.store.lists_mut();
        if !lists_mut.contains_key(&self.new_default_lists) && !self.new {
            bail!("list [{}] does not exist", self.new_default_lists);
        }
        lists_mut
            .entry(self.new_default_lists.clone())
            .or_insert_with(TodoMap::new);

        ctx.store.set_default_list(self.new_default_lists);

        ctx.store.write(&ctx.store_path)
    }
}
