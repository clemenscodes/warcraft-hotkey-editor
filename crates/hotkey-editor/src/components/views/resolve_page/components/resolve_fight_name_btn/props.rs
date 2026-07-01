use dioxus::prelude::*;

/// The moved ability's name + id as a button that deep-links into the editor when
/// the ability has a carrier unit.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveFightNameBtnProps {
    #[props(into)]
    pub name: String,
    #[props(into)]
    pub object_id: String,
    pub has_unit: bool,
    pub onclick: EventHandler<MouseEvent>,
}
