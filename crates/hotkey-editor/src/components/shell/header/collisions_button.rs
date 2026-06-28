use dioxus::prelude::*;
use warcraft_keybinds::{CrossUnitCollisionReport, CustomKeys, UnitCollisionReport};

use crate::components::shared::icons::{ICON_COLLISIONS, ICON_COLLISIONS_CLEAR};
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::view_navigation::ViewNavigationContext;
use warcraft_keybinds::GridLayout;

const COLLISIONS_BUTTON_STYLES: Asset =
    asset!("/src/components/shell/header/collisions_button.css");

#[derive(Props, Clone, PartialEq)]
pub struct CollisionsButtonProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub navigation: ViewNavigationContext,
}

/// Toolbar icon that surfaces collision health and routes to the
/// Collisions page on click.  Two visual states, both styled to match
/// the rest of the WoW toolbar chrome — no SaaS-style notification
/// bubbles:
///
/// - **Collisions present** — amber-tinted warning triangle with the
///   count rendered as bold gold text in the corner of the icon (the
///   same pattern WoW uses for spell-stack counters: integrated, not
///   floating).
/// - **All clear** — circled checkmark glowing gold.  Rewards the user
///   for a clean config.
#[component]
pub fn CollisionsButton(props: CollisionsButtonProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let navigation = props.navigation;

    let breakdown = CollisionBreakdown::compute(loaded_keys, grid_layout);
    let collision_count = breakdown.total();
    let cross_unit_count = breakdown.cross_unit;
    let per_unit_position_count = breakdown.per_unit_position;
    let per_unit_hotkey_count = breakdown.per_unit_hotkey;
    let has_collisions = collision_count > 0;
    let count_label = if collision_count >= 100 {
        "99+".to_string()
    } else {
        collision_count.to_string()
    };
    let aria_label = if has_collisions {
        format!("Collisions — {collision_count} to review")
    } else {
        "Collisions — your config is clean".to_string()
    };
    let icon_html = if has_collisions {
        ICON_COLLISIONS
    } else {
        ICON_COLLISIONS_CLEAR
    };
    let button_class = if has_collisions {
        "collisions-button collisions-button--attention"
    } else {
        "collisions-button collisions-button--clear"
    };

    let go_to_collisions = move |_| {
        let target = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        navigation.apply(target);
    };

    rsx! {
        document::Stylesheet { href: COLLISIONS_BUTTON_STYLES }
        button {
            class: button_class,
            r#type: "button",
            "aria-label": "{aria_label}",
            "data-action": "view-collisions",
            "data-collision-count": "{collision_count}",
            "data-collision-cross-unit": "{cross_unit_count}",
            "data-collision-per-unit-position": "{per_unit_position_count}",
            "data-collision-per-unit-hotkey": "{per_unit_hotkey_count}",
            "data-collision-state": if has_collisions { "attention" } else { "clear" },
            onclick: go_to_collisions,
            // The count rides as a top-right corner overlay inside the button
            // box (absolute-positioned) so the icon never shifts off-center.
            span {
                class: "collisions-button-icon",
                aria_hidden: "true",
                dangerous_inner_html: icon_html,
            }
            if has_collisions {
                span {
                    class: "collisions-button-badge",
                    "data-collision-badge": "true",
                    aria_hidden: "true",
                    "{count_label}"
                }
            }
        }
    }
}

/// Per-class breakdown of every collision the badge surfaces.
/// Exposed as discrete fields so the renderer can publish each class
/// as its own `data-*` attribute — useful for e2e regression tests
/// and for debugging discrepancies between the badge label and what
/// the user intuitively expects to see.
struct CollisionBreakdown {
    /// Cross-unit position groups — cells where two or more units
    /// share an ability and at least one unit has a multi-button
    /// collision there.
    cross_unit: usize,
    /// Per-unit position collisions — cells on a single unit's
    /// command card where two or more of its abilities land at the
    /// same slot.
    per_unit_position: usize,
    /// Per-unit hotkey collisions — letters on a single unit's
    /// command card claimed by two or more buttons.  Includes
    /// ability-vs-ability AND ability-vs-Cmd*-system-command conflicts
    /// (Cmd* slots live on the command card so they show up here).
    per_unit_hotkey: usize,
}

impl CollisionBreakdown {
    fn compute(loaded_keys: Signal<Option<CustomKeys>>, grid_layout: Signal<GridLayout>) -> Self {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return Self {
                cross_unit: 0,
                per_unit_position: 0,
                per_unit_hotkey: 0,
            };
        };
        let layout = *grid_layout.read();

        let cross_unit_report = CrossUnitCollisionReport::compute(file);
        let cross_unit = cross_unit_report.position_groups().len();

        let unit_report = UnitCollisionReport::compute(file, layout);
        let mut per_unit_position: usize = 0;
        let mut per_unit_hotkey: usize = 0;
        for entry in unit_report.entries() {
            for card in entry.position_cards() {
                per_unit_position += card.into_iter().count();
            }
            for card in entry.hotkey_cards() {
                per_unit_hotkey += card.into_iter().count();
            }
        }

        Self {
            cross_unit,
            per_unit_position,
            per_unit_hotkey,
        }
    }

    fn total(&self) -> usize {
        self.cross_unit + self.per_unit_position + self.per_unit_hotkey
    }
}
