use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::UnitKind;

pub(super) struct UnitCategoryHeadingInputs {
    pub(super) category_kind: UnitKind,
    pub(super) is_collapsed: bool,
    pub(super) collapsed_categories: Signal<HashSet<UnitKind>>,
}

pub(super) struct CategoryHeadingData {
    pub(super) label: String,
    pub(super) is_collapsed: bool,
    pub(super) on_toggle: EventHandler<MouseEvent>,
}

impl From<UnitCategoryHeadingInputs> for CategoryHeadingData {
    fn from(inputs: UnitCategoryHeadingInputs) -> Self {
        let category_kind = inputs.category_kind;
        let label = category_kind.category_label().to_owned();
        let is_collapsed = inputs.is_collapsed;
        let mut collapsed_categories = inputs.collapsed_categories;
        let on_toggle = EventHandler::new(move |_event: MouseEvent| {
            let mut categories = collapsed_categories.write();
            if categories.contains(&category_kind) {
                categories.remove(&category_kind);
            } else {
                categories.insert(category_kind);
            }
        });
        Self {
            label,
            is_collapsed,
            on_toggle,
        }
    }
}
use super::model::UnitCategorySectionModel;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use crate::services::editor_state::context::use_editor_state;
use warcraft_api::WarcraftObjectId;

pub(super) struct UnitCardEntry {
    pub(super) unit_id: WarcraftObjectId,
    pub(super) display_name: String,
    pub(super) icon_path: Option<IconUrl>,
    pub(super) unit_kind: UnitKind,
}

pub(super) struct UnitCategorySectionPresentation {
    pub(super) heading: CategoryHeadingData,
    pub(super) is_collapsed: bool,
    pub(super) cards: Vec<UnitCardEntry>,
}

/// Shapes the section from the group it was handed.
///
/// It asks the catalog for nothing. Every section used to re-read the race, the
/// mode, the query, the search field and the visibility from context and run its
/// own database pass, so a four-category list scanned the whole catalog four
/// times per keystroke — on top of the pass that produced the categories. The
/// units now arrive as data and this only turns them into cards. The one signal
/// it still reads is which categories are collapsed, which is UI state, not a
/// fetch.
pub(super) fn use_unit_category_section(
    props: &UnitCategorySectionModel,
) -> UnitCategorySectionPresentation {
    let group = props.group.clone();
    let category_kind = group.category_kind();
    let editor = use_editor_state();
    let collapsed_categories = editor.collapsed_categories();
    let is_collapsed = collapsed_categories.read().contains(&category_kind);
    let heading_inputs = UnitCategoryHeadingInputs {
        category_kind,
        is_collapsed,
        collapsed_categories,
    };
    let heading = CategoryHeadingData::from(heading_inputs);
    let entries = group.into_entries();
    let cards = entries
        .into_iter()
        .map(|entry| {
            let icon_path = entry.icon_database_path().map(IconUrl::from_database_path);
            let unit_id = entry.unit_id();
            let display_name = entry.display_name().to_owned();
            let unit_kind = entry.unit_kind();
            UnitCardEntry {
                unit_id,
                display_name,
                icon_path,
                unit_kind,
            }
        })
        .collect();
    UnitCategorySectionPresentation {
        heading,
        is_collapsed,
        cards,
    }
}
