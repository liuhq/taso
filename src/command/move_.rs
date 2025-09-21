use crate::{command::RunMut, context::Context, model::Todo, util::get_list};
use anyhow::Result;
use clap::Args;
use log::{debug, trace, warn};

#[derive(Debug, Args)]
pub struct MoveOptions {
    todo_id: u32,
    #[command(flatten)]
    todo_move_target: TodoMoveTarget,
    /// Move todo in specified list
    #[arg(short, long, value_name = "LIST_NAME")]
    list: Option<String>,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct TodoMoveTarget {
    /// Move into todo <PARENT_TODO_ID> as a child todo.
    #[arg(long, value_name = "PARENT_TODO_ID", default_value = "")]
    parent: u32,
    #[arg(long)]
    top: bool,
}

impl RunMut for MoveOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        trace!(target: "move", "{self:#?}");

        let list = get_list(self.list, ctx);
        debug!(target: "move", "list [{list}]");

        if self.todo_move_target.top {
            debug!(target: "move", "todo [{}] move to top level", self.todo_id);
            _move_to_top(ctx, &list, &self.todo_id)?;
        } else {
            debug!(target: "move", "todo [{}] will have new parent todo [{}]", self.todo_id, self.todo_move_target.parent);
            _move_to_children(
                ctx,
                &list,
                &self.todo_id,
                &self.todo_move_target.parent,
            )?;
        }

        ctx.store.write(&ctx.store_path)
    }
}

fn _move_to_top(ctx: &mut Context, list: &String, ch_id: &u32) -> Result<()> {
    let ch_todo = ctx.store.todo_by_id_mut(&list, &ch_id)?;
    match ch_todo.parent.clone() {
        Some(pa_id) => {
            debug!(target: "move::_move_to_top", "target todo [{ch_id}] has parent todo [{pa_id}]");
            ch_todo.parent = None;
            debug!(target: "move::_move_to_top", "target todo [{ch_id}] removes parent todo [{pa_id}]");

            _remove_child_id(ctx, list, &pa_id, ch_id)
        }
        None => {
            warn!(target: "move::_move_to_top", "target todo [{}] has no parent todo", ch_id);
            Ok(())
        }
    }
}

fn _move_to_children(
    ctx: &mut Context,
    list: &String,
    ch_id: &u32,
    target_pa_id: &u32,
) -> Result<()> {
    let ch_todo = ctx.store.todo_by_id_mut(&list, &ch_id)?;

    match ch_todo.parent.clone() {
        Some(pa_id) => {
            debug!(target: "move::_move_to_children", "target todo [{ch_id}] has parent todo [{pa_id}]");
            ch_todo.parent = Some(target_pa_id.clone());
            debug!(target: "move::_move_to_children", "target todo [{ch_id}] has new parent todo [{target_pa_id}]");

            _remove_child_id(ctx, list, &pa_id, ch_id)?;
        }
        None => {
            debug!(target: "move::_move_to_children", "target todo [{ch_id}] had no parent todo before");
            ch_todo.parent = Some(target_pa_id.clone());
            debug!(target: "move::_move_to_children", "target todo [{ch_id}] has new parent todo [{target_pa_id}]");
        }
    };

    let pa_todo = ctx.store.todo_by_id_mut(&list, &target_pa_id)?;
    pa_todo.children.get_or_insert_with(Vec::new).push(*ch_id);
    debug!(target: "move::_move_to_children", "target todo [{target_pa_id}] has new child todo [{ch_id}]");
    Ok(())
}

fn _remove_child_id(
    ctx: &mut Context,
    list: &String,
    pa_id: &u32,
    ch_id: &u32,
) -> Result<()> {
    debug!(target: "move::_remove_child_id", "target pre-parent-todo [{pa_id}]");
    let pa_todo = ctx.store.todo_by_id_mut(&list, &pa_id)?;
    match &mut pa_todo.children {
        Some(pa_todo_ch_ids) => {
            debug!(target: "move::_remove_child_id", "target pre-parent-todo [{pa_id}] has children todos [{pa_todo_ch_ids:?}]");
            pa_todo_ch_ids.retain(|id| *id != *ch_id);
            debug!(target: "move::_remove_child_id", "target pre-parent-todo [{pa_id}] remove child todo [{ch_id}]");
            _empty_to_none(pa_todo);
            Ok(())
        }
        None => {
            warn!(target: "move::_remove_child_id", "target pre-parent-todo [{}] has no child todo", pa_id);
            Ok(())
        }
    }
}

fn _empty_to_none(todo: &mut Todo) {
    if todo.children.as_ref().is_some_and(|v| v.is_empty()) {
        todo.children = None;
        debug!(target: "move::_empty_to_none", "set empty children todo to <None>");
    }
}
