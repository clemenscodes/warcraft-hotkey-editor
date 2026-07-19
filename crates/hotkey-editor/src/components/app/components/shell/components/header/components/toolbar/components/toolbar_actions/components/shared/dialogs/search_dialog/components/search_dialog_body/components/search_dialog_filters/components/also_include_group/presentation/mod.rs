use super::data;
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;

pub(super) struct IncludeSwitch {
    pub(super) key: &'static str,
    pub(super) label: &'static str,
    pub(super) popover_text: &'static str,
    pub(super) is_on: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_also_include_group() -> Vec<IncludeSwitch> {
    let editor = use_editor_state();
    let mut show_abilityless = editor.show_abilityless_units();
    let mut expand_variants = editor.expand_variants();
    let abilityless_on = *show_abilityless.read();
    let variants_on = *expand_variants.read();
    let on_abilityless = EventHandler::new(move |_event: MouseEvent| {
        let next = !*show_abilityless.peek();
        show_abilityless.set(next);
    });
    let on_variants = EventHandler::new(move |_event: MouseEvent| {
        let next = !*expand_variants.peek();
        expand_variants.set(next);
    });
    let abilityless = IncludeSwitch {
        key: "abilityless",
        label: data::ABILITYLESS_LABEL,
        popover_text: data::ABILITYLESS_POPOVER,
        is_on: abilityless_on,
        onclick: on_abilityless,
    };
    let variants = IncludeSwitch {
        key: "variants",
        label: data::VARIANTS_LABEL,
        popover_text: data::VARIANTS_POPOVER,
        is_on: variants_on,
        onclick: on_variants,
    };
    vec![abilityless, variants]
}
