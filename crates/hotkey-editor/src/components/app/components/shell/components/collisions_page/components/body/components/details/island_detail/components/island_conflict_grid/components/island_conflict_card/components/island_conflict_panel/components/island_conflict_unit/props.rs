use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The affected unit heading an island conflict card: a big icon, name, and object
/// id that deep-link into the editor focused on that unit. The navigation used to open
/// the unit is read from context, so it is not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictUnitProps {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
}
