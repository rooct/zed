use gpui::{div, Div};

use crate::StyledExt;

/// Horizontally stacks elements.
///
/// Sets `flex()`, `flex_row()`, `items_center()`
#[track_caller]
pub fn h_stack() -> Div {
    div().h_flex()
}

/// Vertically stacks elements.
///
/// Sets `flex()`, `flex_col()`
#[track_caller]
pub fn v_stack() -> Div {
    div().v_flex()
}
