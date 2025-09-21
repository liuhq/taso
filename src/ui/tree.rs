use crate::{
    model::{Todo, TodoMap},
    ui::tree::{hack::TodoText, preset_tree::Tree, widget_node::Node},
};
use anyhow::Result;
use promkit::Prompt;
use std::collections::HashMap;

pub mod hack;
pub mod preset_evaluate;
pub mod preset_tree;
pub mod widget_node;
pub mod widget_tree;

enum StackState {
    Pending,
    Ready,
}

pub struct TreeUI;

impl TreeUI {
    pub async fn run(
        title: &String,
        root_id: &String,
        todos: &TodoMap,
        tree_line: u8,
    ) -> Result<Vec<String>> {
        Tree::new(Node::NonLeaf {
            id: TodoText {
                id: root_id.clone(),
                desc: String::new(),
                link: None,
                complete: None,
            },
            children: build_tree(todos),
            children_visible: true,
        })
        .title(title)
        .tree_lines(tree_line as usize)
        .evaluator(|event, ctx| Box::pin(preset_evaluate::default(event, ctx)))
        .run()
        .await
    }
}

fn build_tree(todos: &TodoMap) -> Vec<Node> {
    let mut nodes = HashMap::new();
    let top_nodes: Vec<_> = todos
        .iter()
        .filter(|(_, todo)| todo.parent.is_none())
        .collect();

    for (top_id, top_todo) in top_nodes {
        let mut stack = Vec::new();
        stack.push((top_id, top_todo, StackState::Pending));
        while let Some((id, todo, state)) = stack.pop() {
            match state {
                StackState::Pending => match &todo.children {
                    Some(ch_ids) => {
                        stack.push((id, todo, StackState::Ready));
                        for ch_id in ch_ids {
                            stack.push((
                                &ch_id,
                                todos.get(ch_id).unwrap(),
                                StackState::Pending,
                            ));
                        }
                    }
                    None => {
                        let leaf = Node::Leaf(desc(id, &todo));
                        nodes.insert(id, leaf);
                    }
                },
                StackState::Ready => {
                    let mut children = Vec::new();
                    for ch_id in todo.children.as_ref().unwrap() {
                        let ch_node = nodes.remove_entry(ch_id).unwrap();
                        children.push(ch_node);
                    }

                    // sort the children nodes
                    children.sort_by_key(|(id, _)| *id);
                    let children =
                        children.into_iter().map(|(_, t)| t).collect();
                    let non_leaf = Node::NonLeaf {
                        id: desc(id, &todo),
                        children,
                        children_visible: true,
                    };

                    nodes.insert(id, non_leaf);
                }
            }
        }
    }

    // sort the top nodes
    let mut nodes: Vec<_> = nodes.into_iter().collect();
    nodes.sort_by_key(|(id, _)| *id);
    nodes.into_iter().map(|(_, n)| n).collect()
}

fn desc(todo_id: &u32, todo: &Todo) -> TodoText {
    TodoText {
        id: todo_id.to_string(),
        desc: todo.desc.clone(),
        link: todo.link.clone(),
        complete: todo.complete_at.map(|c| c.to_string()),
    }
}
