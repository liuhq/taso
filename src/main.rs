use anyhow::Result;
use clap::Parser;
use log::{debug, error, info, trace};
use taso::{
    command::{Cli, Cmd, Run, RunMut, TodoCmd},
    config::Config,
    context::Context,
    store::Store,
    util::search_or_upward,
};

fn main() -> Result<()> {
    env_logger::init();
    trace!(target: "main", "env_logger initialized");

    match run() {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    trace!(target: "main::run", "clap cli parsed");
    debug!(target: "main::run", "{cli:#?}");
    let config = Config::read()?;
    trace!(target: "main::run", "config read: {config:#?}");

    let Some(cmd) = cli.cmd else {
        info!("{:#?}", cli);
        return Ok(());
    };

    match cmd {
        Cmd::Init(init) => init.run(&config, cli.global),
        Cmd::Todo(todo_cmd) => {
            let mut ctx = init_ctx(config, cli.global)?;

            match todo_cmd {
                TodoCmd::Add(add) => add.run_mut(&mut ctx),
                TodoCmd::Tree(tree) => tree.run(&ctx),
                TodoCmd::Track(track) => track.run(&ctx),
                TodoCmd::Modify(modify) => modify.run_mut(&mut ctx),
                TodoCmd::Move(move_) => move_.run_mut(&mut ctx),
                TodoCmd::Remove(remove) => remove.run_mut(&mut ctx),
                TodoCmd::Done(done) => done.run_mut(&mut ctx),
                TodoCmd::Clean(clean) => clean.run_mut(&mut ctx),
                TodoCmd::ListShow(l_show) => l_show.run(&ctx),
                TodoCmd::ListAdd(l_add) => l_add.run_mut(&mut ctx),
                TodoCmd::ListRemove(l_remove) => l_remove.run_mut(&mut ctx),
                TodoCmd::ListDefault(l_default) => l_default.run_mut(&mut ctx),
            }
        }
    }
}

fn init_ctx(config: Config, global: bool) -> Result<Context> {
    let store_path = if global {
        config.global_store().join(config.data_file_name())
    } else {
        match search_or_upward(config.data_file_name())? {
            Some(p) => p,
            None => config.global_store().join(config.data_file_name()),
        }
    };
    info!(target: "main::init_ctx", "store path: {:?}", store_path);
    let store = Store::read(&store_path, &config)?;
    trace!(target: "main::init_ctx", "store read");
    let ctx = Context::new(config, store_path, store);
    trace!(target: "main::init_ctx", "context initialized");

    Ok(ctx)
}
