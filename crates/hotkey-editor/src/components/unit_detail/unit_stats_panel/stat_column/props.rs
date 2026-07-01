use super::state::StatColumnKind;
use dioxus::prelude::*;

/// One stat category column: which category it is (its grid area), whether it
/// carries an icon (row layout), and its content.
#[derive(Props, Clone, PartialEq)]
pub struct StatColumnProps {
    pub kind: StatColumnKind,
    #[props(default)]
    pub with_icon: bool,
    pub children: Element,
}
