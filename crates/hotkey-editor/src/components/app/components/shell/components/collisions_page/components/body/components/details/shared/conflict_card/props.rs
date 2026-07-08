use crate::components::app::components::shell::components::shared::panel_card::{
    PanelCardProps, PanelCardVariant,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictCardProps {
    pub children: Element,
}

impl From<&ConflictCardProps> for PanelCardProps {
    fn from(props: &ConflictCardProps) -> Self {
        let children = props.children.clone();
        Self {
            variant: PanelCardVariant::Conflict,
            children,
        }
    }
}
