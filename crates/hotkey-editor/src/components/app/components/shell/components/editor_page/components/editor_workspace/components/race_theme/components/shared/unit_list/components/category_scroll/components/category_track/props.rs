use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// The track's props: the kinds of the category sections to lay out, in display order.
#[derive(Props, Clone, PartialEq)]
pub struct CategoryTrackProps {
    pub sections: Vec<UnitKind>,
}
