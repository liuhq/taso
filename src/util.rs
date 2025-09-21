use crate::context::Context;
use anyhow::{Result, bail};
use log::debug;
use std::{collections::HashMap, env, fmt::Display, ops::Deref, path::PathBuf};
use tokio::runtime::Runtime;

pub fn search_or_upward(
    file_name: impl AsRef<str> + Display,
) -> Result<Option<PathBuf>> {
    debug!(target: "util::search_or_upward", "file: {file_name}");
    let start_dir = env::current_dir()?;
    debug!(target: "util::search_or_upward", "start at: {start_dir:?}");
    let mut current_dir = Some(start_dir.as_path());

    while let Some(p) = current_dir {
        debug!(target: "util::search_or_upward", "current at: {current_dir:?}");
        let target = p.join(file_name.as_ref());
        if target.exists() {
            debug!(target: "util::search_or_upward", "find target file: {target:?}");
            return Ok(Some(target));
        }
        current_dir = p.parent();
    }

    Ok(None)
}

pub fn expand_tilde(path: PathBuf) -> PathBuf {
    match path.strip_prefix("~") {
        Ok(rest) => match env::home_dir().map(|p| p.join(rest)) {
            Some(home_rest) => home_rest,
            None => path,
        },
        Err(_) => path,
    }
}

pub fn get_list(arg: Option<String>, ctx: &Context) -> String {
    match arg {
        Some(list) => {
            debug!(target: "util::get_list", "use list from arg: {list:?}");
            list
        }
        None => {
            let default_list = ctx.store.default_list();
            if default_list.is_empty() {
                let config_default_list = ctx.config.default_list().to_owned();
                debug!(target: "util::get_list", "use default list from config: {config_default_list}");
                config_default_list
            } else {
                debug!(target: "util::get_list", "use default list from store: {default_list}");
                default_list.to_owned()
            }
        }
    }
}

pub fn check_key<K, V>(hash: &HashMap<K, V>, key: &K) -> Result<()>
where
    K: ToString + Display,
    K: std::hash::Hash + std::cmp::Eq,
{
    debug!(target: "util::check_key", "key [{key}]");
    if hash.contains_key(&key) {
        Ok(())
    } else {
        bail!("key [{key}] does not exist")
    }
}

pub struct SyncBlock(pub Runtime);

impl Deref for SyncBlock {
    type Target = Runtime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SyncBlock {
    pub fn new() -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        Ok(Self(runtime))
    }
}

pub trait IntoOption {
    type Target;
    /// Returns `None` if this `String` has a length of zero, and `Some(String)` otherwise.
    fn into_option(self) -> Option<Self::Target>;
}

impl IntoOption for String {
    type Target = String;

    fn into_option(self) -> Option<Self::Target> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
