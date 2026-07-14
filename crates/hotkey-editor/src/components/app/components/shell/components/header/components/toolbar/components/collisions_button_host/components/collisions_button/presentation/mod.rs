use super::model::CollisionsButtonModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::SurfaceState;
use crate::components::app::components::shell::components::shared::icons::{
    ICON_COLLISIONS, ICON_COLLISIONS_CLEAR,
};
use dioxus::prelude::*;

pub struct CollisionsButtonPresentation {
    pub(super) surface_state: SurfaceState,
    pub(super) collision_count: usize,
    pub(super) count_label: String,
    pub(super) aria_label: String,
    pub(super) icon: &'static str,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&CollisionsButtonModel> for CollisionsButtonPresentation {
    fn from(props: &CollisionsButtonModel) -> Self {
        let summary = props.summary;
        let onclick = props.onclick;
        let collision_count = summary.total();
        let has_collisions = !summary.is_clean();
        let surface_state = if has_collisions {
            SurfaceState::Attention
        } else {
            SurfaceState::Clear
        };
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
        let icon = if has_collisions {
            ICON_COLLISIONS
        } else {
            ICON_COLLISIONS_CLEAR
        };
        Self {
            surface_state,
            collision_count,
            count_label,
            aria_label,
            icon,
            onclick,
        }
    }
}

impl ddd::Presentation for CollisionsButtonPresentation {
    type Model = CollisionsButtonModel;
}
