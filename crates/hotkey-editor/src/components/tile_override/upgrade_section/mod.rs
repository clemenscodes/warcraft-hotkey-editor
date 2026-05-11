use dioxus::prelude::*;

use super::key_field::OverrideKeyField;

#[derive(Props, Clone, PartialEq)]
pub(super) struct UpgradeSectionProps {
    pub(super) upgrade_hotkey_label: String,
    pub(super) upgrade_is_editing: bool,
    pub(super) upgrade_hotkey_is_special: bool,
    pub(super) is_research_context: bool,
    pub(super) on_position_click: EventHandler<()>,
    pub(super) on_hotkey_activate: EventHandler<()>,
}

#[component]
pub(super) fn UpgradeSection(props: UpgradeSectionProps) -> Element {
    let upgrade_hotkey_label = props.upgrade_hotkey_label;
    let upgrade_is_editing = props.upgrade_is_editing;
    let upgrade_hotkey_is_special = props.upgrade_hotkey_is_special;
    let is_research_context = props.is_research_context;
    let on_position_click = props.on_position_click;
    let on_hotkey_activate = props.on_hotkey_activate;
    let _ = is_research_context;
    rsx! {
        div { class: "tile-override-alt-state",
            div { class: "tile-override-alt-state-header",
                div { class: "tile-override-alt-state-header-text",
                    p { class: "tile-override-alt-state-label", "Upgraded form" }
                }
                button {
                    class: "tile-override-alt-state-position-button",
                    r#type: "button",
                    title: "Pick where the upgraded-form button appears on the command card",
                    aria_label: "Edit upgraded-form button position",
                    onclick: move |_| on_position_click.call(()),
                    svg {
                        class: "tile-override-alt-state-position-icon",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        circle { cx: "12", cy: "12", r: "5", fill: "none", stroke: "currentColor", stroke_width: "1.6" }
                        line { x1: "12", y1: "2.5", x2: "12", y2: "6", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                        line { x1: "12", y1: "18", x2: "12", y2: "21.5", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                        line { x1: "2.5", y1: "12", x2: "6", y2: "12", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                        line { x1: "18", y1: "12", x2: "21.5", y2: "12", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                        circle { cx: "12", cy: "12", r: "1.4", fill: "currentColor" }
                    }
                }
                OverrideKeyField {
                    label: upgrade_hotkey_label,
                    is_editing: upgrade_is_editing,
                    is_special: upgrade_hotkey_is_special,
                    title: String::from("Hotkey for the upgraded form"),
                    on_activate: move |_| on_hotkey_activate.call(()),
                }
            }
        }
    }
}
