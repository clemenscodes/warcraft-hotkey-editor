use dioxus::prelude::*;

use crate::components::dialogs::help_dialog::data::HelpContent;

/// The body's input: the whole guide content, threaded down to the sections.
#[derive(Props, Clone, PartialEq)]
pub struct HelpBodyProps {
    pub content: HelpContent,
}
