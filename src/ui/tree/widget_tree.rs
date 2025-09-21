//! Based on https://github.com/ynqa/promkit/blob/main/promkit-widgets/src/tree.rs commit aebd817ffc108c7364e16a8608d375f1a976b1d0
//! Based on https://github.com/ynqa/promkit/blob/main/promkit-widgets/src/tree/tree.rs commit b0fa9f162d35314dd6df0a808242f1e5b5529da6

use crate::ui::tree::{
    hack::{TodoStyle, TodoText},
    widget_node::{Kind, Node},
};
use promkit::core::{
    Pane, PaneFactory,
    crossterm::style::{ContentStyle, Stylize},
    grapheme::StyledGraphemes,
};
use promkit::widgets::cursor::Cursor;
use std::cell::Cell;

/// Represents the state of a tree structure within the application.
///
/// This state includes not only the tree itself but also various properties
/// that affect how the tree is displayed and interacted with. These properties
/// include symbols for folded and unfolded items, styles for active and inactive
/// items, the number of lines available for rendering, and the indentation level
/// for child items in the tree.
pub struct State {
    pub tree: Tree,

    /// Symbol representing folded items.
    pub folded_symbol: String,
    /// Symbol representing unfolded items.
    pub unfolded_symbol: String,
    /// Style for symbol.
    pub symbol_style: ContentStyle,

    /// Style for the selected line.
    pub active_item_style: TodoStyle,
    /// Style for un-selected lines.
    pub inactive_item_style: TodoStyle,

    /// Number of lines available for rendering.
    pub lines: Option<usize>,

    /// The number of spaces used for indenting child items in the tree.
    /// This value determines how much horizontal space is used to visually
    /// represent the hierarchical structure of the tree. Each level of
    /// indentation typically represents a deeper level in the tree hierarchy.
    pub indent: usize,

    list_start: Cell<usize>,
    list_end: Cell<usize>,
}

impl State {
    pub fn new(root: Node) -> Self {
        Self {
            tree: Tree::new(root),
            // folded_symbol: String::from("▶︎ "),
            // unfolded_symbol: String::from("▼ "),
            folded_symbol: String::from("+ "),
            unfolded_symbol: String::from("- "),
            symbol_style: ContentStyle::new().blue(),
            active_item_style: TodoStyle::active_defautl_style(),
            inactive_item_style: TodoStyle::inactive_defautl_style(),
            lines: Default::default(),
            indent: 2,
            list_start: Default::default(),
            list_end: Default::default(),
        }
    }

    pub fn set_list_end(&mut self, list_end: usize) {
        self.list_end.set(list_end);
    }
}

impl PaneFactory for State {
    fn create_pane(&self, width: u16, height: u16) -> Pane {
        let symbol = |kind: &Kind| -> &str {
            match kind {
                Kind::Folded { .. } => &self.folded_symbol,
                Kind::Unfolded { .. } => &self.unfolded_symbol,
            }
        };

        let indent = |kind: &Kind| -> usize {
            match kind {
                Kind::Folded { path, .. } | Kind::Unfolded { path, .. } => {
                    // prevent indent of root
                    (path.len() - 1) * self.indent
                }
            }
        };

        let id = |kind: &Kind| -> TodoText {
            match kind {
                Kind::Folded { id, .. } | Kind::Unfolded { id, .. } => {
                    id.clone()
                }
            }
        };

        let height = match self.lines {
            Some(lines) => lines.min(height as usize),
            None => height as usize,
        };

        if self.tree.position() >= self.list_end.get() {
            self.list_start.set(self.list_start.get() + 1);
            self.list_end.set(self.list_end.get() + 1);
        }

        if self.tree.position() < self.list_start.get() {
            self.list_start.set(self.list_start.get().saturating_sub(1));
            self.list_end.set(self.list_end.get() - 1);
        }

        let matrix = self
            .tree
            .kinds()
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                *i >= self.list_start.get() && *i < self.list_end.get()
            })
            .map(|(i, kind)| {
                if i == self.tree.position() {
                    StyledGraphemes::from_iter([
                        StyledGraphemes::from_str(
                            symbol(kind),
                            self.symbol_style,
                        ),
                        StyledGraphemes::from_str(
                            " ".repeat(indent(kind)),
                            ContentStyle::default(),
                        ),
                        TodoStyle::format_items(
                            &self.active_item_style,
                            id(kind),
                            true,
                        ),
                    ])
                } else {
                    StyledGraphemes::from_iter([
                        StyledGraphemes::from_str(
                            " ".repeat(
                                StyledGraphemes::from(symbol(kind)).widths(),
                            ),
                            self.symbol_style,
                        ),
                        StyledGraphemes::from_str(
                            " ".repeat(indent(kind)),
                            ContentStyle::default(),
                        ),
                        TodoStyle::format_items(
                            &self.inactive_item_style,
                            id(kind),
                            false,
                        ),
                    ])
                }
            })
            .fold((vec![], 0), |(mut acc, pos), item| {
                let rows = item.matrixify(width as usize, height, 0).0;
                if pos < self.tree.position() + height {
                    acc.extend(rows);
                }
                (acc, pos + 1)
            });

        Pane::new(matrix.0, 0)
    }
}

//----------------------------------------------------------------------
//----------------------------------------------------------------------

/// A `Tree` structure that manages a collection of nodes in a hierarchical manner.
/// It utilizes a cursor to navigate and manipulate the nodes within the tree.
#[derive(Clone)]
pub struct Tree {
    root: Node,
    cursor: Cursor<Vec<Kind>>,
}

impl Tree {
    /// Creates a new `Tree` with a given root node.
    ///
    /// # Arguments
    ///
    /// * `root` - The root node of the tree.
    pub fn new(root: Node) -> Self {
        Self {
            root: root.clone(),
            cursor: Cursor::new(Self::node_into_cursor(&root), 0, false),
        }
    }

    /// Flatten tree node and remove root node
    fn node_into_cursor(root: &Node) -> Vec<Kind> {
        root.flatten_visibles().into_iter().skip(1).collect()
    }

    /// Returns a vector of all nodes in the tree, represented with their depth information.
    pub fn kinds(&self) -> Vec<Kind> {
        self.cursor.contents().clone()
    }

    /// Returns the current position of the cursor within the tree.
    pub fn position(&self) -> usize {
        self.cursor.position()
    }

    /// Retrieves the data of the current node pointed by the cursor, along with its path from the root.
    pub fn get(&self) -> Vec<String> {
        let kind = self.cursor.contents()[self.position()].clone();
        match kind {
            Kind::Folded { id, path } | Kind::Unfolded { id, path } => {
                let mut ret = self.root.get_waypoints(&path);
                ret.push(id.id);
                ret
            }
        }
    }

    /// Toggles the state of the current node and updates the cursor position accordingly.
    pub fn toggle(&mut self) {
        let path = match self.cursor.contents()[self.position()].clone() {
            Kind::Folded { path, .. } => path,
            Kind::Unfolded { path, .. } => path,
        };
        self.root.toggle(&path);
        self.cursor = Cursor::new(
            Self::node_into_cursor(&self.root),
            self.position(),
            false,
        );
    }

    /// Moves the cursor backward in the tree, if possible.
    ///
    /// Returns `true` if the cursor was successfully moved backward, `false` otherwise.
    pub fn backward(&mut self) -> bool {
        self.cursor.backward()
    }

    /// Moves the cursor forward in the tree, if possible.
    ///
    /// Returns `true` if the cursor was successfully moved forward, `false` otherwise.
    pub fn forward(&mut self) -> bool {
        self.cursor.forward()
    }

    /// Moves the cursor to the head of the tree.
    pub fn move_to_head(&mut self) {
        self.cursor.move_to_head()
    }

    /// Moves the cursor to the tail of the tree.
    pub fn move_to_tail(&mut self) {
        self.cursor.move_to_tail()
    }
}
