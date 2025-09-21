use crate::context::Context;
use anyhow::Result;

pub trait Run {
    fn run(self, ctx: &Context) -> Result<()>;
}

pub trait RunMut {
    fn run_mut(self, ctx: &mut Context) -> Result<()>;
}

pub mod add;
pub mod clean;
pub mod done;
pub mod init;
pub mod list_add;
pub mod list_default;
pub mod list_remove;
pub mod list_show;
pub mod modify;
pub mod move_;
pub mod remove;
pub mod track;
pub mod tree;

/*
*
* Clap CLI
*
*/

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Output the result without performing any actual operation
    #[arg(long, hide = true)]
    pub dry_run: bool,
    /// Specify global todolists
    #[arg(short, long)]
    pub global: bool,
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Create a new .todo file in current working directory
    Init(init::InitOptions),
    #[command(flatten)]
    Todo(TodoCmd),
}

#[derive(Debug, Subcommand)]
pub enum TodoCmd {
    /// Add a new todo.
    ///
    /// interactive mode when --desc and --link options are missing.
    Add(add::AddOptions),
    /// List all todos in specified DATE_OPTION.
    Tree(tree::TreeOptions),
    /// Show detailed information of a todo.
    Track(track::TrackOptions),
    /// Modify an existing todo.
    ///
    /// interactive mode when --desc and --link options are missing.
    Modify(modify::ModifyOptions),
    /// Change the level of a todo and its children.
    Move(move_::MoveOptions),
    /// Remove an exist todo(s), <TODO_ID> must have no children.
    Remove(remove::RemoveOptions),
    /// Mark a todo(s) as DONE, <TODO_ID> must have no children.
    Done(done::DoneOptions),
    /// Clean up all completed todos in specified DATE_OPTION.
    Clean(clean::CleanOptions),
    /// Show all lists.
    ListShow(list_show::ListShowOptions),
    /// Add lists.
    ListAdd(list_add::ListAddOptions),
    /// Remove lists.
    ListRemove(list_remove::ListRemoveOptions),
    /// Set default list.
    ListDefault(list_default::ListDefaultOptions),
}
