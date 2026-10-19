mod model;
mod view;

mod style;

use super::shared::regen_mark::RegenMark;
use dioxus::prelude::*;
use model::RegenConditionNoteModel;
use style::CLASS;
use tw_macro::assert_component;

/// Spells out what the icon at a conditional regeneration means, right below
/// the table, so the icon never has to be guessed at or hovered.
#[component]
pub fn RegenConditionNote(props: RegenConditionNoteModel) -> Element {
    let regen_type = props.regen.regen_type();
    let Some(mark) = RegenMark::for_regen_type(regen_type) else {
        return rsx! {};
    };
    let icon = mark.icon();
    let meaning = mark.meaning();
    rsx! {
        p {
            class: CLASS,
            span {
                aria_hidden: "true",
                dangerous_inner_html: icon,
            }
            {meaning}
        }
    }
}

assert_component!(RegenConditionNote);
