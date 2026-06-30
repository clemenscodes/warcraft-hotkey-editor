use dioxus::prelude::*;

use super::components::help_glossary_entry::HelpGlossaryEntryProps;

/// One column's only input: the glossary entries it lays out, in order.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryColumnProps {
    pub entries: &'static [HelpGlossaryEntryProps],
}
