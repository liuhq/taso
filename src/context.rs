use std::path::PathBuf;

use crate::{config::Config, store::Store};

#[derive(Debug)]
pub struct Context {
    pub config: Config,
    pub store_path: PathBuf,
    pub store: Store,
}

impl Context {
    pub fn new(config: Config, store_path: PathBuf, store: Store) -> Self {
        Self {
            store_path,
            store,
            config,
        }
    }
}
