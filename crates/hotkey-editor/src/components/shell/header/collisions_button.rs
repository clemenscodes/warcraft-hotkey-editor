use dioxus::prelude::*;
use warcraft_keybinds::{CrossUnitCollisionReport, CustomKeys, UnitCollisionReport};

use crate::components::shared::icons::{ICON_COLLISIONS, ICON_COLLISIONS_CLEAR};
use crate::model::grid::GridLayout;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::view_navigation::ViewNavigationContext;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct CollisionsButtonProps {
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) grid_layout: Signal<GridLayout>,
    pub(crate) navigation: ViewNavigationContext,
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
pub(crate) fn CollisionsButton(props: CollisionsButtonProps) -> Element {
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
        BUTTON_ATTENTION_CLASS
    } else {
        BUTTON_CLEAR_CLASS
    };

    let go_to_collisions = move |_| {
        let target = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        navigation.apply(target);
    };

    rsx! {
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
            // Icon centered exactly like every other toolbar button —
            // matches `TOOLBAR_ICON_CLASS` sizing so the row looks
            // aligned.  The count rides as a top-right corner overlay
            // *inside* the button box (absolute-positioned) so the
            // icon never shifts off-center.
            span {
                class: "flex items-center justify-center \
                        [width:calc(2.2rem_*_var(--hdr-scale))] [height:calc(2.2rem_*_var(--hdr-scale))] \
                        leading-none [&_svg]:block [&_svg]:w-full [&_svg]:h-full \
                        max-[1099px]:[width:1.4rem] max-[1099px]:[height:1.4rem]",
                aria_hidden: "true",
                dangerous_inner_html: icon_html,
            }
            if has_collisions {
                span {
                    class: "absolute top-[calc(0.4rem_*_var(--hdr-scale))] right-[calc(0.45rem_*_var(--hdr-scale))] \
                            font-mono font-bold leading-none \
                            text-[#ffe39a] \
                            [font-size:calc(1rem_*_var(--hdr-scale))] \
                            [text-shadow:1px_1px_0_rgba(0,0,0,0.95),-1px_1px_0_rgba(0,0,0,0.95),1px_-1px_0_rgba(0,0,0,0.95),-1px_-1px_0_rgba(0,0,0,0.95),0_0_3px_rgba(0,0,0,0.95)] \
                            pointer-events-none \
                            max-[1099px]:!top-[5px] max-[1099px]:!right-[6px] \
                            max-[1099px]:text-[0.9rem]",
                    "data-collision-badge": "true",
                    aria_hidden: "true",
                    "{count_label}"
                }
            }
        }
    }
}

/// Attention state — collisions present.  Same chrome as every other
/// toolbar button (no extra glow, no off-palette colors); the warmer
/// amber icon + count below it carry the alert.  Mobile sizing matches
/// the burger menu (44px square, standard touch target).
const BUTTON_ATTENTION_CLASS: &str = "relative inline-flex items-center justify-center shrink-0 \
     [width:calc(5rem_*_var(--hdr-scale))] [height:calc(5rem_*_var(--hdr-scale))] p-0 \
     [background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)] \
     border border-[#6c5a1f] [border-radius:calc(12px_*_var(--hdr-scale))] \
     text-[#e8a23a] cursor-pointer \
     [transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease] \
     [@media(hover:hover)]:hover:border-warcraft-gold \
     [@media(hover:hover)]:hover:text-warcraft-gold \
     [@media(hover:hover)]:hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)] \
     [@media(hover:hover)]:hover:[box-shadow:0_0_12px_rgba(255,206,99,0.3)] \
     focus:outline-none \
     focus-visible:border-white focus-visible:text-white \
     focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] \
     max-[1099px]:w-[44px] max-[1099px]:h-[44px] \
     max-[1099px]:min-w-[44px] max-[1099px]:min-h-[44px] \
     max-[1099px]:rounded-[10px]";

/// Clear state — zero collisions.  Same chrome but the border switches
/// to full gold and the icon glows gold; reads as the affirmative
/// "achievement" indicator without any decorative extras.
const BUTTON_CLEAR_CLASS: &str = "relative inline-flex items-center justify-center shrink-0 \
     [width:calc(5rem_*_var(--hdr-scale))] [height:calc(5rem_*_var(--hdr-scale))] p-0 \
     [background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)] \
     border border-warcraft-gold [border-radius:calc(12px_*_var(--hdr-scale))] \
     text-warcraft-gold cursor-pointer \
     [box-shadow:0_0_10px_rgba(255,206,99,0.2)] \
     [transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease] \
     [@media(hover:hover)]:hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)] \
     [@media(hover:hover)]:hover:[box-shadow:0_0_14px_rgba(255,206,99,0.45)] \
     focus:outline-none \
     focus-visible:border-white focus-visible:text-white \
     focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] \
     max-[1099px]:w-[44px] max-[1099px]:h-[44px] \
     max-[1099px]:min-w-[44px] max-[1099px]:min-h-[44px] \
     max-[1099px]:rounded-[10px]";

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
