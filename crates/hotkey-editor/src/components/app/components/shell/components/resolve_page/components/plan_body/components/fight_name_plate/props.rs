use dioxus::prelude::*;
/// The rival ability's name + id (non-interactive plate).
#[derive(Props, Clone, PartialEq)]
pub struct FightNamePlateProps {
    #[props(into)]
    pub name: String,
    #[props(into)]
    pub object_id: String,
}
