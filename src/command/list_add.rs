use crate::{command::RunMut, context::Context, model::TodoMap};
use anyhow::Result;
use clap::Args;
use log::debug;

#[derive(Debug, Args)]
pub struct ListAddOptions {
    new_lists: Vec<String>,
}

impl RunMut for ListAddOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        debug!(target: "list_add", "{ctx:#?}");
        let lists_mut = ctx.store.lists_mut();
        for new_list in self.new_lists {
            let list_map = TodoMap::new();
            lists_mut.insert(new_list, list_map);
        }

        ctx.store.write(&ctx.store_path)
    }
}
