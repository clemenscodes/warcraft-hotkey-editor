use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels};

use crate::model::grid::DragFollower;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct DragFollowerOverlayProps {
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) active_race: Signal<Race>,
}

#[component]
pub(crate) fn DragFollowerOverlay(props: DragFollowerOverlayProps) -> Element {
    let drag_follower = props.drag_follower;
    let active_race = props.active_race;
    let follower_option = drag_follower.read().clone();
    let Some(follower) = follower_option else {
        return rsx! {};
    };
    let visual = follower.visual();
    let style_value = format!(
        "left: {left}px; top: {top}px; width: {width}px; height: {height}px;",
        left = follower.left(),
        top = follower.top(),
        width = follower.tile_width(),
        height = follower.tile_height(),
    );
    let mut class_name = String::from("drag-follower");
    if visual.is_command_cell() {
        class_name.push_str(" is-command");
    }
    let hotkey_overlay_class = if visual.is_passive_command() {
        "hotkey-overlay passive"
    } else {
        "hotkey-overlay"
    };
    let race_attribute = RaceLabels::data_attribute(*active_race.read());
    rsx! {
        div { class: class_name, "data-race": race_attribute, style: style_value,
            if let Some(source) = visual.icon_source() {
                img {
                    src: source,
                    alt: "{visual.label_text()}",
                    draggable: "false",
                    decoding: "async",
                }
            } else {
                span { class: "command-label", "{visual.label_text()}" }
            }
            if let Some(letter_text) = visual.displayed_letter() {
                span { class: hotkey_overlay_class, {letter_text} }
            }
        }
    }
}
