use crate::components::app::components::shell::components::shared::panel_card::{
    PanelCardProps, PanelCardVariant,
};
use dioxus::prelude::*;

/// The shared move-card shell; `is_stuck` tints the border for unresolved cards.
#[derive(Props, Clone, PartialEq)]
pub struct MoveCardProps {
    #[props(default)]
    pub is_stuck: bool,
    pub children: Element,
}

impl From<&MoveCardProps> for PanelCardProps {
    fn from(props: &MoveCardProps) -> Self {
        let variant = if props.is_stuck {
            PanelCardVariant::MoveStuck
        } else {
            PanelCardVariant::Move
        };
        let children = props.children.clone();
        Self { variant, children }
    }
}
