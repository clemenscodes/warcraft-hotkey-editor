use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub(super) struct HeroLevelOptionProps {
    pub(super) level_index: u32,
    pub(super) current_level: u32,
    pub(super) selected_hero_level: Signal<u32>,
    pub(super) level_picker_open: Signal<bool>,
}

#[component]
pub(super) fn HeroLevelOption(props: HeroLevelOptionProps) -> Element {
    let level_index = props.level_index;
    let current_level = props.current_level;
    let mut selected_hero_level = props.selected_hero_level;
    let mut level_picker_open = props.level_picker_open;
    let is_active = level_index == current_level;
    let option_class = if is_active {
        "hero-level-option active"
    } else {
        "hero-level-option"
    };
    let handle_select = move |_| {
        selected_hero_level.set(level_index);
        level_picker_open.set(false);
    };
    rsx! {
        button {
            class: option_class,
            r#type: "button",
            onclick: handle_select,
            "Level {level_index}"
        }
    }
}
