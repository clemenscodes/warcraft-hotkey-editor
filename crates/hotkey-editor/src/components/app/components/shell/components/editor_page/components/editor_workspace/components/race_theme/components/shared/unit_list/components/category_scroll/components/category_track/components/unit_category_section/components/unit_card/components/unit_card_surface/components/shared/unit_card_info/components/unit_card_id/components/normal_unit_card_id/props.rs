use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id the muted code element renders.
#[derive(Props, Clone, PartialEq)]
pub struct NormalUnitCardIdProps {
    pub unit_id: WarcraftObjectId,
}
