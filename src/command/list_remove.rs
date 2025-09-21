use crate::{command::RunMut, context::Context, model::Todo, util::check_key};
use anyhow::Result;
use clap::Args;
use log::{debug, warn};

#[derive(Debug, Args)]
pub struct ListRemoveOptions {
    remove_lists: Vec<String>,
    #[arg(short, long)]
    force: bool,
}

impl RunMut for ListRemoveOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        debug!(target: "list_remove", "{ctx:#?}");
        let mut remove_lists = Vec::new();
        let mut remove_failed = Vec::new();
        let mut migrate_todos: Vec<(u32, Todo)> = Vec::new();

        let default_list = ctx.store.default_list().clone();
        debug!(target: "list_remove", "default list [{default_list}]");

        let lists = ctx.store.lists_mut();
        for list in self.remove_lists {
            if list == default_list {
                warn!("list [{list}] is default list, can't be removed");
                remove_failed.push(list);
                continue;
            }
            check_key(lists, &list)?;
            let todos = lists.get(&list).unwrap();
            if todos.len() > 0 {
                if !self.force {
                    warn!("list [{list}] still have todo, can't be removed!");
                    remove_failed.push(list);
                    continue;
                }
                let todos = lists.remove(&list).unwrap();
                migrate_todos.extend(todos.into_iter());
            }
            remove_lists.push(list);
        }

        let lists_mut = ctx.store.lists_mut();
        for list in remove_lists {
            debug!(target: "list_remove", "list [{list}] is removed");
            lists_mut.remove(&list);
        }

        let default_todos = ctx.store.todos_mut(&default_list)?;
        default_todos.extend(migrate_todos.into_iter());

        if remove_failed.len() > 0 {
            println!("Failed: {remove_failed:?}");
        }

        ctx.store.write(&ctx.store_path)
    }
}
