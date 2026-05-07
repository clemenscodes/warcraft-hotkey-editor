use dioxus::prelude::*;

const MAX_HERO_LEVEL_DISPLAY: u32 = 10;

#[component]
pub(crate) fn UnitDetailHeader(
    unit_name: &'static str,
    unit_id: String,
    portrait_url: Option<String>,
    has_hero_attributes: bool,
    selected_hero_level: Signal<u32>,
    level_picker_open: Signal<bool>,
) -> Element {
    let current_level = selected_hero_level();
    rsx! {
        header { class: "unit-detail-header",
            if let Some(source) = portrait_url {
                img {
                    class: "unit-portrait",
                    src: "{source}",
                    alt: "{unit_name}",
                    loading: "lazy",
                    decoding: "async",
                }
            }
            div { class: "unit-detail-title",
                div { class: "unit-name-row",
                    h2 { "{unit_name}" }
                    if has_hero_attributes {
                        div { class: "hero-level-picker",
                            button {
                                class: if level_picker_open() { "hero-level-trigger open" } else { "hero-level-trigger" },
                                r#type: "button",
                                onclick: move |_| level_picker_open.set(!level_picker_open()),
                                span { class: "hero-level-trigger-label", "Level" }
                                span { class: "hero-level-trigger-number", "{current_level}" }
                                span { class: "hero-level-trigger-chevron", "▾" }
                            }
                            if level_picker_open() {
                                div { class: "hero-level-menu",
                                    for level_index in 1..=MAX_HERO_LEVEL_DISPLAY {
                                        {
                                            let is_active = level_index == current_level;
                                            rsx! {
                                                button {
                                                    key: "{level_index}",
                                                    class: if is_active { "hero-level-option active" } else { "hero-level-option" },
                                                    r#type: "button",
                                                    onclick: move |_| {
                                                        selected_hero_level.set(level_index);
                                                        level_picker_open.set(false);
                                                    },
                                                    "Level {level_index}"
                                                }
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "hero-level-backdrop",
                                    onclick: move |_| level_picker_open.set(false),
                                }
                            }
                        }
                    }
                }
                code { "{unit_id}" }
            }
        }
    }
}
