use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// One carrier of an ability in the carriers dialog: an icon, name, and id that
/// deep-link into the editor focused on that unit. The navigation used to open the unit
/// is read from context, so it is not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardProps {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
}
