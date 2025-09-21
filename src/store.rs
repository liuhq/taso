use crate::model::{Todo, TodoMap};
use crate::{config::Config, util::check_key};
use anyhow::Result;
use log::debug;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

type Lists = HashMap<String, TodoMap>;

#[derive(Clone, Deserialize, Serialize)]
pub struct Store {
    default_list: String,
    #[serde(
        flatten,
        serialize_with = "serialize_lists",
        deserialize_with = "deserialize_lists"
    )]
    lists: Lists,
}

impl Store {
    pub fn default_list(&self) -> &String {
        &self.default_list
    }

    pub fn set_default_list(&mut self, default_list: String) {
        self.default_list = default_list;
    }

    pub fn lists(&self) -> &Lists {
        &self.lists
    }

    pub fn lists_mut(&mut self) -> &mut Lists {
        &mut self.lists
    }

    pub fn todos(&self, list: &String) -> Result<&TodoMap> {
        debug!(target: "store::todos", "list [{list}]");
        check_key(&self.lists, list)?;
        Ok(self.lists.get(list).unwrap())
    }

    pub fn todos_mut(&mut self, list: &String) -> Result<&mut TodoMap> {
        debug!(target: "store::todos_mut", "list [{list}]");
        check_key(&self.lists, list)?;
        Ok(self.lists.get_mut(list).unwrap())
    }

    pub fn todo_by_id(&self, list: &String, todo_id: &u32) -> Result<&Todo> {
        debug!(target: "store::todo_by_id", "list [{list}]");
        debug!(target: "store::todo_by_id", "todo [{todo_id}]");
        let todos = self.todos(list)?;
        check_key(todos, todo_id)?;
        Ok(todos.get(todo_id).unwrap())
    }

    pub fn todo_by_id_mut(
        &mut self,
        list: &String,
        todo_id: &u32,
    ) -> Result<&mut Todo> {
        debug!(target: "store::todo_by_id_mut", "list [{list}]");
        debug!(target: "store::todo_by_id_mut", "todo [{todo_id}]");
        let todos_mut = self.todos_mut(list)?;
        check_key(todos_mut, todo_id)?;
        Ok(todos_mut.get_mut(todo_id).unwrap())
    }

    fn max_id(&self, list: &String) -> Result<&u32> {
        debug!(target: "store::max_id", "list [{list}]");
        Ok(self.todos(list)?.keys().max().unwrap_or(&0))
    }

    pub fn generate_id(&self, list: &String) -> Result<u32> {
        debug!(target: "store::generate_id", "list [{list}]");
        Ok(self.max_id(list)? + 1)

        // if self.allow_uppercase {
        //     loop {
        //         let id: String = rand::rng()
        //             .sample_iter(Alphanumeric)
        //             .take(6)
        //             .map(char::from)
        //             .collect();
        //
        //         if !self.todos(&list)?.contains_key(&id) {
        //             return Ok(id);
        //         }
        //     }
        // } else {
        //     const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        //     loop {
        //         let mut rng = rand::rng();
        //         let id: String = (0..6)
        //             .map(|_| {
        //                 CHARSET[rng.random_range(0..CHARSET.len())] as char
        //             })
        //             .collect();
        //
        //         if !self.todos(&list)?.contains_key(&id) {
        //             return Ok(id);
        //         }
        //     }
        // }
    }
}

impl Store {
    pub fn create(default_list: String, lists_option: Vec<String>) -> Self {
        let lists = std::iter::once(default_list.clone())
            .chain(lists_option)
            .map(|g| (g, TodoMap::new()))
            .collect();
        debug!(target: "store::create", "initialized lists: {lists:#?}");
        Self {
            default_list,
            lists,
        }
    }

    pub fn read<P: AsRef<Path> + Debug>(
        path: P,
        config: &Config,
    ) -> Result<Self> {
        debug!(target: "store::read", "store at: {path:?}");
        debug!(target: "store::read", "config: {config:#?}");
        let content = fs::read_to_string(path)?;
        debug!(target: "store::read", "file content: {content}");

        // create a new store if the file is empty
        Ok(toml::from_str(&content).unwrap_or_else(|_| {
            let new =
                Store::create(config.default_list().to_owned(), Vec::new());
            debug!(target: "store::read", "create new store: {new:#?}");
            new
        }))
    }

    pub fn write(&self, path: &PathBuf) -> Result<()> {
        debug!(target: "store::write", "store at: {path:?}");
        fs::write(path, toml::to_string(self)?)?;
        Ok(())
    }
}

impl Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store")
            .field("default_list", &self.default_list)
            .field(
                "lists",
                &self
                    .lists
                    .iter()
                    .map(|(k, v)| (k, format!("{} item(s) {{...}}", v.len())))
                    .collect::<HashMap<_, _>>(),
            )
            .finish()
    }
}

fn serialize_lists<S>(lists: &Lists, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let sorted: BTreeMap<_, _> = lists.iter().map(|(k, v)| (k, v)).collect();
    let mut map = serializer.serialize_map(Some(sorted.len()))?;
    for (k, v) in sorted {
        map.serialize_entry(k, &v)?;
    }
    map.end()
}

fn deserialize_lists<'de, D>(deserializer: D) -> Result<Lists, D::Error>
where
    D: Deserializer<'de>,
{
    HashMap::deserialize(deserializer)
}
