//! Based on https://github.com/ynqa/promkit/blob/main/promkit/src/preset/tree/evaluate.rs commit 1ba1e9156c5493b308b7b41a7e66bb40648bf391

use crate::ui::tree::preset_tree::Tree;
use promkit::{
    Signal,
    core::crossterm::{
        // self,
        event::{
            Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
        },
    },
};

/// Default key bindings for the tree.
///
/// | Key                    | Action
/// | :--------------------- | :-------------------------------------------
/// | <kbd>Enter</kbd>       | Exit the tree view
/// | <kbd>q</kbd>       | Exit the tree view
/// | <kbd>Ctrl + C</kbd>    | Interrupt the current operation
/// | <kbd>k</kbd>           | Move the selection up
/// | <kbd>j</kbd>           | Move the selection down
/// | <kbd>Space</kbd>       | Toggle fold/unfold at the current node
pub async fn default(event: &Event, ctx: &mut Tree) -> anyhow::Result<Signal> {
    match event {
        // Render for refreshing prompt on resize.
        Event::Resize(width, height) => {
            ctx.render(*width, *height).await?;
        }

        // Quit
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Ok(Signal::Quit),
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Ok(Signal::Quit),
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Err(anyhow::anyhow!("ctrl+c")),

        // Move cursor.
        Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            ctx.tree.tree.backward();
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            ctx.tree.tree.forward();
        }

        // Fold/Unfold
        Event::Key(KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            ctx.tree.tree.toggle();
        }

        // Event::Key(KeyEvent {
        //     code: KeyCode::Char('r'),
        //     modifiers: KeyModifiers::NONE,
        //     kind: KeyEventKind::Press,
        //     state: KeyEventState::NONE,
        // }) => {
        //     let size = crossterm::terminal::size()?;
        //     ctx.render(size.0, size.1).await?;
        // }
        _ => (),
    }
    Ok(Signal::Continue)
}
