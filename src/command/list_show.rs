use crate::{command::Run, context::Context};
use anyhow::Result;
use clap::Args;
use log::debug;

#[derive(Debug, Args)]
pub struct ListShowOptions;

impl Run for ListShowOptions {
    fn run(self, ctx: &Context) -> Result<()> {
        debug!(target: "list_show", "{ctx:#?}");
        let mut lists: Vec<&String> = ctx.store.lists().keys().collect();
        lists.sort();
        for list in lists {
            let is_default = if list == ctx.store.default_list() {
                "*"
            } else {
                " "
            };
            println!(" {is_default} {list}");
        }
        Ok(())
    }
}
