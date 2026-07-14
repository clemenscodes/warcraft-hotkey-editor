use super::view::AbilityIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The database object id shown under the name.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityIdModel {
    pub object_id: WarcraftObjectId,
}

impl From<&AbilityIdView> for AbilityIdModel {
    fn from(view: &AbilityIdView) -> Self {
        let AbilityIdView { object_id } = view.clone();
        Self { object_id }
    }
}

impl ddd::Model for AbilityIdModel {
    type View = AbilityIdView;
}
