use crate::{
    command::Run,
    context::Context,
    ui::tree::TreeUI,
    util::{SyncBlock, get_list},
};
use anyhow::Result;
use clap::Args;
use log::{debug, trace};

#[derive(Debug, Args)]
pub struct TreeOptions {
    #[command(flatten)]
    rel_date: Option<RelDateOptions>,
    #[command(flatten)]
    abs_date: Option<AbsDateOptions>,
    /// Filter incomplete todos
    #[arg(long, conflicts_with = "done")]
    todo: bool,
    /// Filter completed todos
    #[arg(long)]
    done: bool,
    /// Reverse sort order
    #[arg(short = 'R', long)]
    reverse: bool,
    /// List todos in specified list
    #[arg(short, long, value_name = "LIST_NAME")]
    list: Option<String>,
}

impl Run for TreeOptions {
    fn run(self, ctx: &Context) -> Result<()> {
        trace!(target: "tree", "{self:#?}");

        let list = get_list(self.list, ctx);
        debug!(target: "tree", "list [{list}]");

        let todos = ctx.store.todos(&list)?;

        // TODO: TUI
        trace!(target: "tree", "TODO [tui]");

        let rt = SyncBlock::new()?;
        let result = rt.block_on(TreeUI::run(
            &String::from(ctx.store_path.to_string_lossy()),
            &list,
            todos,
            ctx.config.tree_line(),
        ))?;

        println!("{result:?}");

        // let max_desc_len =
        //     todos.iter().map(|(_, v)| v.desc.len()).max().unwrap_or(0);
        // debug!(target: "tree", "plain text table (max_desc_len): {max_desc_len}");
        // let mut entries: Vec<_> = todos.iter().map(|(k, v)| (k, v)).collect();
        //
        // entries.sort_by(|(k1, v1), (k2, v2)| {
        //     v1.create_at()
        //         .cmp(&v2.create_at())
        //         .then(v1.desc.cmp(&v2.desc))
        //         .then(k1.cmp(k2))
        // });
        //
        // println!(
        //     "\nTodolists stored at {}\n",
        //     ctx.store_path.to_string_lossy()
        // );
        // println!(
        //     "  {}  {:<8}  {:<max_desc_len$}  {:<10}",
        //     " ", "ID", "Todo", "Create At"
        // );
        // println!("  {:-<width$}", "", width = 8 + 2 + max_desc_len + 2 + 12);
        // for (k, v) in entries {
        //     let mark = if v.complete_at == None {
        //         " "
        //     } else {
        //         "*"
        //     };
        //
        //     println!(
        //         "  {}  ({:^6})  {:<max_desc_len$}  {}",
        //         mark,
        //         k,
        //         v.desc,
        //         v.create_at(),
        //     );
        // }

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
