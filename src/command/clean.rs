use crate::{
    command::RunMut,
    context::Context,
    util::{check_key, get_list},
};
use anyhow::Result;
use chrono::Local;
use clap::Args;
use log::{debug, trace, warn};

#[derive(Debug, Args)]
pub struct CleanOptions {
    #[command(flatten)]
    rel_date: Option<RelDateOptions>,
    #[command(flatten)]
    abs_date: Option<AbsDateOptions>,
    /// Clean up todos in specified list
    #[arg(short, long, value_name = "LIST_NAME")]
    list: Option<String>,
}

impl RunMut for CleanOptions {
    fn run_mut(self, ctx: &mut Context) -> Result<()> {
        trace!(target: "clean", "{self:#?}");

        let list = get_list(self.list, ctx);
        debug!(target: "clean", "list [{list}]");

        let before = Local::now().date_naive();
        debug!(target: "clean", "before date: {before}");

        let todos = ctx.store.todos(&list)?;
        let mut clean_id = Vec::new();

        for (id, todo) in todos.iter() {
            let Some(complete_at) = todo.complete_at else {
                debug!(target: "clean", "todo [{id}] to be completed");
                continue;
            };
            debug!(target: "clean", "todo [{id}] {complete_at} before {before}: {}", complete_at <= before);
            if complete_at > before {
                debug!(target: "clean", "todo [{id}] is skipped");
                continue;
            }
            clean_id.push(id.clone());
        }

        debug!(target: "clean", "to be cleaned: todo [{clean_id:?}]");
        if clean_id.len() <= 0 {
            debug!(target: "clean", "nothing be cleaned");
            return Ok(());
        }

        let todos_mut = ctx.store.todos_mut(&list)?;
        let mut cleaned_todos = Vec::new();

        for id in &clean_id {
            check_key(todos_mut, id)?;
            let Some(cleaned) = todos_mut.remove(id) else {
                warn!(target: "clean", "[{id}] can't remove");
                continue;
            };
            cleaned_todos.push(cleaned);
        }

        println!("{:#?}", cleaned_todos);

        ctx.store.write(&ctx.store_path)?;
        Ok(())
    }
}

#[derive(Debug, Args)]
#[group(multiple = false, conflicts_with = "AbsDateOptions")]
#[command(next_help_heading = "Relative Date")]
pub struct RelDateOptions {
    /// (default: 0) Relative date in DAY. <+OFFSET> indicates future and <-OFFSET> is past
    #[arg(
                short = 'd',
                long,
                value_name = "OFFSET",
                allow_negative_numbers = true,
                num_args = 0..=1,
                default_missing_value = "0",
                next_line_help = true
            )]
    pub day_rel: Option<i32>,
    /// (default: 0) Relative date in WEEK. Same as --day_rel
    #[arg(
                short = 'w',
                long,
                value_name = "OFFSET",
                allow_negative_numbers = true,
                num_args = 0..=1,
                default_missing_value = "0"
            )]
    pub week_rel: Option<i32>,
    /// (default: 0) Relative date in MONTH. Same as --day_rel
    #[arg(
                short = 'm',
                long,
                value_name = "OFFSET",
                allow_negative_numbers = true,
                num_args = 0..=1,
                default_missing_value = "0"
            )]
    pub month_rel: Option<i32>,
    /// (default: 0) Relative date in YEAR. Same as --day_rel
    #[arg(
                short = 'y',
                long,
                value_name = "OFFSET",
                allow_negative_numbers = true,
                num_args = 0..=1,
                default_missing_value = "0"
            )]
    pub year_rel: Option<i32>,
}

#[derive(Debug, Args)]
#[group(multiple = true)]
#[command(next_help_heading = "Absolute Date")]
pub struct AbsDateOptions {
    /// Specify a date
    #[arg(
                short = 'D',
                long,
                value_name = "YYYY-MM-DD",
                num_args = 1,
                conflicts_with_all = ["month", "year"],
                next_line_help = true
            )]
    pub date: Option<String>,
    /// <N> must be 1~12 indicating the N-th month of a year, and the current year is assumed by default
    #[arg(short = 'M', long, value_name = "N", num_args = 1)]
    pub month: Option<u32>,
    /// (default: current year) <N> must be a positive integer indicating the specific year
    #[arg(short = 'Y', long, value_name = "N", num_args = 1)]
    pub year: Option<u32>,
}
