mod hero_level_option;

use dioxus::prelude::*;

use hero_level_option::HeroLevelOption;

const MAX_HERO_LEVEL_DISPLAY: u32 = 10;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct UnitDetailHeaderProps {
    pub(crate) unit_name: &'static str,
    pub(crate) unit_id: String,
    pub(crate) portrait_url: Option<String>,
    pub(crate) has_hero_attributes: bool,
    pub(crate) selected_hero_level: Signal<u32>,
    pub(crate) level_picker_open: Signal<bool>,
}

#[component]
pub(crate) fn UnitDetailHeader(props: UnitDetailHeaderProps) -> Element {
    let unit_name = props.unit_name;
    let unit_id = props.unit_id;
    let portrait_url = props.portrait_url;
    let has_hero_attributes = props.has_hero_attributes;
    let selected_hero_level = props.selected_hero_level;
    let mut level_picker_open = props.level_picker_open;
    let current_level = selected_hero_level();
    let current_level_text = current_level.to_string();
    let picker_is_open = level_picker_open();
    let trigger_class = if picker_is_open {
        "hero-level-trigger open"
    } else {
        "hero-level-trigger"
    };
    let toggle_level_picker = move |_| level_picker_open.set(!level_picker_open());
    let close_level_picker = move |_| level_picker_open.set(false);
    rsx! {
        header { class: "unit-detail-header",
            if let Some(source) = portrait_url {
                img {
                    class: "unit-portrait",
                    src: source,
                    alt: unit_name,
                    loading: "lazy",
                    decoding: "async",
                }
            }
            div { class: "unit-detail-title",
                div { class: "unit-name-row",
                    h2 { {unit_name} }
                    if has_hero_attributes {
                        div { class: "hero-level-picker",
                            button {
                                class: trigger_class,
                                r#type: "button",
                                onclick: toggle_level_picker,
                                span { class: "hero-level-trigger-label", "Level" }
                                span { class: "hero-level-trigger-number", {current_level_text} }
                                span { class: "hero-level-trigger-chevron", "▾" }
                            }
                            if picker_is_open {
                                div { class: "hero-level-menu",
                                    for level_index in 1..=MAX_HERO_LEVEL_DISPLAY {
                                        HeroLevelOption {
                                            key: "{level_index}",
                                            level_index,
                                            current_level,
                                            selected_hero_level,
                                            level_picker_open,
                                        }
                                    }
                                }
                                div {
                                    class: "hero-level-backdrop",
                                    onclick: close_level_picker,
                                }
                            }
                        }
                    }
                }
                code { {unit_id} }
            }
        }
    }
}
