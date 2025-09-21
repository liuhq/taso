use crate::{command::RunMut, context::Context, util::get_list};
use anyhow::Result;
use chrono::{Local, NaiveDate};
use clap::Args;
use log::{debug, trace, warn};
use std::collections::HashSet;

#[derive(Debug, Args)]
pub struct DoneOptions {
    #[arg(required = true, value_delimiter = ',')]
    todo_id: Vec<u32>,
    /// Undo the completed todo(s)
    #[arg(short, long)]
    undo: bool,
    /// Mark exist todo(s) and its (or their) children as DONE recursively
    #[arg(short, long)]
    recursive: bool,
    /// Mark todo(s) in specified list
    #[arg(short, long, value_name = "LIST_NAME")]
    list: Option<String>,
}

impl RunMut for DoneOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        trace!(target: "done", "{self:#?}");

        let list = get_list(self.list, ctx);
        debug!(target: "done", "list [{list}]");

        let todo_id = if self.recursive {
            _recursive_todo(&ctx, &list, &self.todo_id)?
        } else {
            self.todo_id
        };
        debug!(target: "done", "todo [{todo_id:?}]");

        let (success, skip) = if self.undo {
            _done(ctx, &list, todo_id, None)?
        } else {
            _done(ctx, &list, todo_id, Some(Local::now().date_naive()))?
        };
        debug!(target: "done", "success todo [{success:?}]");
        debug!(target: "done", "skip todo [{skip:?}]");

        println!("Done:");
        println!("{success:?}");

        if skip.len() > 0 {
            println!("Skip:");
            println!("{skip:?}");
        }

        ctx.store.write(&ctx.store_path)?;
        Ok(())
    }
}

fn _done(
    ctx: &mut Context,
    list: &String,
    todo_id: Vec<u32>,
    complete_at: Option<NaiveDate>,
) -> Result<(Vec<u32>, Vec<u32>)> {
    trace!(target: "done::_done", "{todo_id:?}");
    debug!(target: "done::_done", "complete at: {complete_at:?}");

    let mut result = Vec::new();
    let mut skip = Vec::new();

    for id in todo_id.into_iter() {
        debug!(target: "done::_done", "todo [{id}]");
        let todo = ctx.store.todo_by_id_mut(list, &id)?;
        if complete_at != None && todo.complete_at != None {
            debug!(target: "done::_done", "skip todo [{id}]");
            skip.push(id);
            continue;
        }
        todo.complete_at = complete_at;
        result.push(id);
    }

    Ok((result, skip))
}

fn _recursive_todo(
    ctx: &Context,
    list: &String,
    todo_id: &Vec<u32>,
) -> Result<Vec<u32>> {
    trace!(target: "done::_recursive_todo", "{todo_id:?}");

    let todos = ctx.store.todos(list)?;
    let mut result: Vec<u32> = Vec::new();
    let mut visited: HashSet<u32> = HashSet::new();
    let mut stack: Vec<&u32> = todo_id.iter().collect();

    while let Some(id) = stack.pop() {
        debug!(target: "done::recursive_todo", "todo [{id}] out stack");
        if visited.contains(id) {
            warn!(target: "done::recursive_todo", "todo [{id}] has already been visited");
            continue;
        }
        let Some(todo) = todos.get(id) else {
            warn!(target: "done::recursive_todo", "todo [{id}] is not exist");
            continue;
        };
        visited.insert(id.clone());
        debug!(target: "done::recursive_todo", "todo [{id}] is visited");
        if let Some(ch_ids) = &todo.children {
            for ch_id in ch_ids {
                if visited.contains(ch_id) {
                    warn!(target: "done::recursive_todo", "child todo id [{ch_id}] has already been visited");
                    continue;
                }
                stack.push(ch_id);
                debug!(target: "done::recursive_todo", "child todo id [{ch_id}] into stack");
            }
        }
        result.push(*id);
    }

    Ok(result)
}
