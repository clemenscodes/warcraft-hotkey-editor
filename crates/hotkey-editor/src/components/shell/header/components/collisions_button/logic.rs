use super::state::CollisionState;
use super::style;
use crate::components::shared::icons::{ICON_COLLISIONS, ICON_COLLISIONS_CLEAR};
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::view_navigation::ViewNavigationContext;
use crate::styling::ClassList;
use dioxus::prelude::*;
use warcraft_keybinds::CollisionSummary;

/// Everything the button renders, shaped once from the domain-computed collision
/// summary: the styled class for its state, each collision class's count
/// (published as a `data-*` attribute for e2e), the badge label, the aria label,
/// the state attribute, the icon glyph, and the click handler. The body never
/// computes any of this — it destructures this and places the values.
pub struct CollisionsButtonPresentation {
    pub class: ClassList,
    pub collision_count: usize,
    pub cross_unit_count: usize,
    pub per_unit_position_count: usize,
    pub per_unit_hotkey_count: usize,
    pub count_label: String,
    pub aria_label: String,
    pub state_attribute: &'static str,
    pub icon: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl CollisionsButtonPresentation {
    /// Shape the button from the domain summary and the navigation context. The
    /// counting already happened in `warcraft-keybinds`; this only maps counts to
    /// presentation.
    pub fn build(summary: CollisionSummary, navigation: ViewNavigationContext) -> Self {
        let collision_count = summary.total();
        let cross_unit_count = summary.cross_unit();
        let per_unit_position_count = summary.per_unit_position();
        let per_unit_hotkey_count = summary.per_unit_hotkey();
        let has_collisions = !summary.is_clean();
        let state = if has_collisions {
            CollisionState::Attention
        } else {
            CollisionState::Clear
        };
        let class = style::class(state);
        let count_label = if collision_count >= 100 {
            String::from("99+")
        } else {
            collision_count.to_string()
        };
        let aria_label = if has_collisions {
            format!("Collisions \u{2014} {collision_count} to review")
        } else {
            String::from("Collisions \u{2014} your config is clean")
        };
        let state_attribute = if has_collisions { "attention" } else { "clear" };
        let icon = if has_collisions {
            ICON_COLLISIONS
        } else {
            ICON_COLLISIONS_CLEAR
        };
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            let target = AppView::Collisions {
                kind: CollisionKind::Positions,
            };
            navigation.apply(target);
        });
        Self {
            class,
            collision_count,
            cross_unit_count,
            per_unit_position_count,
            per_unit_hotkey_count,
            count_label,
            aria_label,
            state_attribute,
            icon,
            onclick,
        }
    }
}
