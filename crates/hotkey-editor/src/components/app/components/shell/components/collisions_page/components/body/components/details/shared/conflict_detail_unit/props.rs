use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictDetailUnitProps {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
}
