use chrono::NaiveDate;
use log::trace;
use serde::de::Error;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub desc: String,
    pub link: Option<String>,
    pub children: Option<Vec<u32>>,
    pub parent: Option<u32>,
    create_at: NaiveDate,
    pub complete_at: Option<NaiveDate>,
}

impl Todo {
    pub fn new(
        desc: String,
        link: Option<String>,
        children: Option<Vec<u32>>,
        parent: Option<u32>,
        create_at: NaiveDate,
        complete_at: Option<NaiveDate>,
    ) -> Self {
        let todo = Self {
            desc,
            link,
            children,
            parent,
            create_at,
            complete_at,
        };
        trace!(target: "model::new", "create new Todo: {todo:#?}");
        todo
    }

    pub fn create_at(&self) -> NaiveDate {
        self.create_at
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = &self.desc;
        let link = self
            .link
            .as_ref()
            .map_or_else(|| "(none)".to_owned(), |u| u.clone());
        let create_at = self.create_at();
        let complete_at = self
            .complete_at
            .map_or_else(|| "(todo)".to_owned(), |d| d.to_string());
        let children = self.children.as_ref().map_or_else(
            || "(none)".to_owned(),
            |ch_id| {
                ch_id
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            },
        );
        let parent = self
            .parent
            .as_ref()
            .map_or_else(|| "(none)".to_owned(), |pa_id| pa_id.to_string());

        write!(
            f,
            "Todo: {desc}\nLink: {link}\nCreate At: {create_at}\nComplete At: {complete_at}\nChildren: {children}\nParent: {parent}"
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TodoMap {
    #[serde(
        flatten,
        serialize_with = "serialize_todos",
        deserialize_with = "deserialize_todos"
    )]
    todos: HashMap<u32, Todo>,
}

impl TodoMap {
    pub fn new() -> Self {
        Self {
            todos: HashMap::new(),
        }
    }
}

impl Deref for TodoMap {
    type Target = HashMap<u32, Todo>;

    fn deref(&self) -> &Self::Target {
        &self.todos
    }
}

impl DerefMut for TodoMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.todos
    }
}

impl IntoIterator for TodoMap {
    type Item = (u32, Todo);

    type IntoIter = std::collections::hash_map::IntoIter<u32, Todo>;

    fn into_iter(self) -> Self::IntoIter {
        self.todos.into_iter()
    }
}

/// toml table key can't be a number or numeric string without quote
fn serialize_todos<S>(
    todos: &HashMap<u32, Todo>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut sorted: Vec<_> = todos.iter().collect();
    sorted.sort_by_key(|(id, _)| *id);
    let mut map = serializer.serialize_map(Some(sorted.len()))?;
    for (k, v) in sorted {
        map.serialize_entry(&format!("#{k}"), v)?;
    }
    map.end()
}

fn deserialize_todos<'de, D>(
    deserializer: D,
) -> Result<HashMap<u32, Todo>, D::Error>
where
    D: Deserializer<'de>,
{
    let parsed: HashMap<String, _> = HashMap::deserialize(deserializer)?;
    parsed
        .into_iter()
        .map(|(k, v)| -> Result<(u32, Todo), D::Error> {
            Ok((
                k.strip_prefix("#")
                    .ok_or_else(|| {
                        D::Error::custom(format!("invalid key format: {}", k))
                    })?
                    .parse::<u32>()
                    .map_err(|err| {
                        D::Error::custom(format!("invalid u32: ({})", err))
                    })?,
                v,
            ))
        })
        .collect()
}
