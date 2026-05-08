use dioxus::prelude::*;

use super::key_field::OverrideKeyField;

#[derive(Props, Clone, PartialEq)]
pub(super) struct AltStateSectionProps {
    pub(super) alt_name_text: Option<String>,
    pub(super) alt_description_lines: Vec<String>,
    pub(super) show_alt_controls: bool,
    pub(super) alt_hotkey_label: String,
    pub(super) alt_hotkey_is_editing: bool,
    pub(super) alt_hotkey_is_special_token: bool,
    pub(super) on_position_click: EventHandler<()>,
    pub(super) on_hotkey_activate: EventHandler<()>,
}

#[component]
pub(super) fn AltStateSection(props: AltStateSectionProps) -> Element {
    let alt_name_text = props.alt_name_text;
    let alt_description_lines = props.alt_description_lines;
    let show_alt_controls = props.show_alt_controls;
    let alt_hotkey_label = props.alt_hotkey_label;
    let alt_hotkey_is_editing = props.alt_hotkey_is_editing;
    let alt_hotkey_is_special_token = props.alt_hotkey_is_special_token;
    let on_position_click = props.on_position_click;
    let on_hotkey_activate = props.on_hotkey_activate;
    rsx! {
        div { class: "tile-override-alt-state",
            div { class: "tile-override-alt-state-header",
                div { class: "tile-override-alt-state-header-text",
                    if let Some(alt_name) = alt_name_text {
                        p { class: "tile-override-alt-state-label", {alt_name} }
                    }
                }
                if show_alt_controls {
                    button {
                        class: "tile-override-alt-state-position-button",
                        r#type: "button",
                        title: "Pick where the off-state button appears on the command card",
                        aria_label: "Edit off-state button position",
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
                        label: alt_hotkey_label,
                        is_editing: alt_hotkey_is_editing,
                        is_special: alt_hotkey_is_special_token,
                        title: String::from("Hotkey for the off state (writes Unhotkey)"),
                        on_activate: move |_| on_hotkey_activate.call(()),
                    }
                }
            }
            for description_line in alt_description_lines {
                p { class: "tile-override-alt-state-line", {description_line} }
            }
        }
    }
}
