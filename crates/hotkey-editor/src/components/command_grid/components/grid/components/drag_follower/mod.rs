mod style;

use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels};

use crate::components::command_grid::{HotkeyBadge, HotkeyBadgeState};
use crate::model::grid::DragFollower;
use style::DRAG_FOLLOWER_STYLES;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerOverlayProps {
    pub drag_follower: Signal<Option<DragFollower>>,
    #[props(default = Race::Neutral)]
    pub race: Race,
    /// Whether this grid owns the in-progress drag. The stylesheet is always
    /// emitted (so it is in `<head>` before any drag, avoiding a first-paint
    /// flicker), but the follower element only renders when visible.
    #[props(default)]
    pub visible: bool,
}

#[component]
pub fn DragFollowerOverlay(props: DragFollowerOverlayProps) -> Element {
    let follower_option = if props.visible {
        props.drag_follower.read().clone()
    } else {
        None
    };
    let race_attr = RaceLabels::data_attribute(props.race);
    let follower_element = follower_option.map(|follower| {
        let visual = follower.visual();
        let position_style = format!(
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
        let badge_state = if visual.is_passive_command() {
            HotkeyBadgeState::Passive
        } else {
            HotkeyBadgeState::Normal
        };
        let icon_source = visual.icon_source().map(|icon| icon.to_string());
        let label_text = visual.label_text().to_string();
        let letter = visual.displayed_letter().map(|letter| letter.to_string());
        rsx! {
            div { class: class_name, "data-race": race_attr, style: position_style,
                if let Some(source) = icon_source {
                    img { src: source, alt: label_text, draggable: "false", decoding: "async" }
                } else {
                    span { class: "drag-follower-label", {label_text} }
                }
                if let Some(letter_text) = letter {
                    div { class: "drag-follower-badge",
                        HotkeyBadge { letter: letter_text, state: badge_state }
                    }
                }
            }
        }
    });
    rsx! {
        document::Stylesheet { href: DRAG_FOLLOWER_STYLES }
        {follower_element}
    }
}
