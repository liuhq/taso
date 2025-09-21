use crate::{config::Config, context::Context, store::Store};
use anyhow::{Result, bail};
use clap::Args;
use log::{debug, trace};
use std::env;

#[derive(Debug, Args)]
pub struct InitOptions {
    /// Specify `DEFAULT_LIST_NAME` as default list instead of `default`
    #[arg(long, value_name = "LIST_NAME")]
    default_list: Option<String>,
    /// Create additional list(s) for this data file.
    /// Note: default list will be created automatically
    #[arg(long, value_name = "LIST_NAME", value_delimiter = ',')]
    lists: Vec<String>,
}

impl InitOptions {
    pub fn run(&self, config: &Config, global: bool) -> Result<()> {
        trace!(target: "init", "{self:#?}");

        debug!(target: "init", "{config:#?}");
        let store_path = if global {
            config.global_store().clone().join(config.data_file_name())
        } else {
            println!("{}", config.global_store().exists());
            env::current_dir()?.join(config.data_file_name())
        };
        debug!(target: "init", "target store at: {store_path:?}");
        if store_path.exists() {
            bail!("Todolists store already exists");
        }

        let default_list = self
            .default_list
            .clone()
            .unwrap_or_else(|| config.default_list().to_owned());
        let store = Store::create(default_list, Vec::new());
        debug!(target: "init", "initialized: {store:#?}");

        let ctx = Context::new(config.clone(), store_path, store);
        ctx.store.write(&ctx.store_path)?;
        println!("Initialize at {}", ctx.store_path.to_string_lossy());
        Ok(())
    }
}
