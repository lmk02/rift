use serde::{Deserialize, Serialize};

use crate::Direction;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSelector {
    Index(usize),
    Name(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreScope {
    Workspace,
    Space,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreSource {
    #[default]
    SavedActiveSpace,
    CurrentSpace,
}

/// Cycles through the attached displays in physical order, wrapping at either end.
/// A direction cannot do this: from the rightmost display there is nothing to the right.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayCycle {
    Next,
    Prev,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisplaySelector {
    Direction(Direction),
    /// Must stay ahead of `Uuid`: this is an untagged enum, so "next" would otherwise
    /// deserialize into a UUID that matches no display.
    Cycle(DisplayCycle),
    Index(usize),
    Uuid(String),
}
