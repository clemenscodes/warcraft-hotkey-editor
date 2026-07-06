//! The domain answer to "which unit categories and which first unit does a
//! browse produce". The renderer used to call `UnitCatalog::entries_for` and run
//! the dedupe/first-result loop itself at render time; that is a domain decision
//! (ARCHITECTURE R3), so it lives here. The renderer hands over the raw browse
//! inputs and reads back the already-shaped [`UnitListing`].

use warcraft_api::{Race, UnitKind};
use warcraft_database::{CatalogVisibility, SearchField, UnitCatalog, UnitMode};

/// The inputs to a unit-list browse: the active race and mode, the current
/// search query and the field it searches, and whether hidden units are shown.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitListingRequest {
    race: Race,
    mode: UnitMode,
    search_query: String,
    search_field: SearchField,
    visibility: CatalogVisibility,
}

impl UnitListingRequest {
    /// Build a browse request from the current list-view inputs.
    pub fn new(
        race: Race,
        mode: UnitMode,
        search_query: String,
        search_field: SearchField,
        visibility: CatalogVisibility,
    ) -> Self {
        Self {
            race,
            mode,
            search_query,
            search_field,
            visibility,
        }
    }

    /// Whether a non-empty search is in effect. A search browses across every
    /// race and mode, so it drops the race and mode filters.
    fn is_searching(&self) -> bool {
        !self.search_query.is_empty()
    }
}

/// The first unit a browse yields: its id and its category. The list view
/// selects it when the current selection falls outside the results.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnitListingEntry {
    unit_id: String,
    unit_kind: UnitKind,
}

impl UnitListingEntry {
    pub fn unit_id(&self) -> &str {
        &self.unit_id
    }

    pub fn unit_kind(&self) -> UnitKind {
        self.unit_kind
    }
}

/// The shaped result of a unit-list browse: the category kinds present in the
/// results (deduped, in first-seen order) and the first result, if any.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UnitListing {
    category_kinds: Vec<UnitKind>,
    first_result: Option<UnitListingEntry>,
}

impl UnitListing {
    /// Browse the catalog for the given request and shape the result: walk the
    /// entries once, collecting each distinct category in first-seen order and
    /// remembering the very first entry.
    pub fn resolve(request: &UnitListingRequest) -> Self {
        let searching = request.is_searching();
        let race_filter = if searching { None } else { Some(request.race) };
        let mode_filter = if searching { None } else { Some(request.mode) };
        let query = request.search_query.as_str();
        let query_filter = Some(query);
        let entries = UnitCatalog::entries_for(
            race_filter,
            mode_filter,
            None,
            query_filter,
            request.search_field,
            request.visibility,
        );
        let mut category_kinds: Vec<UnitKind> = Vec::new();
        let mut first_result: Option<UnitListingEntry> = None;
        for entry in entries {
            let entry_kind = entry.unit_kind();
            if first_result.is_none() {
                let unit_id = entry.unit_id().to_owned();
                let first_entry = UnitListingEntry {
                    unit_id,
                    unit_kind: entry_kind,
                };
                first_result = Some(first_entry);
            }
            if !category_kinds.contains(&entry_kind) {
                category_kinds.push(entry_kind);
            }
        }
        Self {
            category_kinds,
            first_result,
        }
    }

    pub fn category_kinds(&self) -> &[UnitKind] {
        &self.category_kinds
    }

    pub fn first_result(&self) -> Option<&UnitListingEntry> {
        self.first_result.as_ref()
    }
}

/// The inputs to browsing one category's units: the same browse inputs as a
/// [`UnitListingRequest`], narrowed to a single [`UnitKind`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitCategoryRequest {
    race: Race,
    mode: UnitMode,
    category_kind: UnitKind,
    search_query: String,
    search_field: SearchField,
    visibility: CatalogVisibility,
}

impl UnitCategoryRequest {
    /// Build a per-category browse request.
    pub fn new(
        race: Race,
        mode: UnitMode,
        category_kind: UnitKind,
        search_query: String,
        search_field: SearchField,
        visibility: CatalogVisibility,
    ) -> Self {
        Self {
            race,
            mode,
            category_kind,
            search_query,
            search_field,
            visibility,
        }
    }

    fn is_searching(&self) -> bool {
        !self.search_query.is_empty()
    }
}

/// One unit in a category browse, shaped for a card: its id and category, its
/// display name (with the "(unnamed)" fallback the domain applies for the rare
/// nameless object), and its icon's database path if it has one. The renderer
/// turns the icon path into a URL — that is presentation, so it stays out here.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnitCategoryEntry {
    unit_id: String,
    unit_kind: UnitKind,
    display_name: String,
    icon_database_path: Option<String>,
}

impl UnitCategoryEntry {
    pub fn unit_id(&self) -> &str {
        &self.unit_id
    }

    pub fn unit_kind(&self) -> UnitKind {
        self.unit_kind
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn icon_database_path(&self) -> Option<&str> {
        self.icon_database_path.as_deref()
    }
}

/// The units of one category, shaped for the list view's cards.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UnitCategoryListing {
    entries: Vec<UnitCategoryEntry>,
}

impl UnitCategoryListing {
    /// Browse the catalog for one category and shape each entry into card data.
    pub fn resolve(request: &UnitCategoryRequest) -> Self {
        let searching = request.is_searching();
        let race_filter = if searching { None } else { Some(request.race) };
        let mode_filter = if searching { None } else { Some(request.mode) };
        let category_filter = Some(request.category_kind);
        let query = request.search_query.as_str();
        let query_filter = Some(query);
        let catalog_entries = UnitCatalog::entries_for(
            race_filter,
            mode_filter,
            category_filter,
            query_filter,
            request.search_field,
            request.visibility,
        );
        let mut entries: Vec<UnitCategoryEntry> = Vec::new();
        for catalog_entry in catalog_entries {
            let warcraft_object = catalog_entry.warcraft_object();
            let names = warcraft_object.names();
            let first_name = names.first().copied().unwrap_or("(unnamed)");
            let display_name = first_name.to_owned();
            let icons = warcraft_object.icons();
            let first_icon = icons.first().copied();
            let icon_database_path = first_icon.map(|icon_path| icon_path.to_owned());
            let unit_id = catalog_entry.unit_id().to_owned();
            let unit_kind = catalog_entry.unit_kind();
            let entry = UnitCategoryEntry {
                unit_id,
                unit_kind,
                display_name,
                icon_database_path,
            };
            entries.push(entry);
        }
        Self { entries }
    }

    pub fn entries(&self) -> &[UnitCategoryEntry] {
        &self.entries
    }

    pub fn into_entries(self) -> Vec<UnitCategoryEntry> {
        self.entries
    }
}

impl ddd::ReadModel for UnitListing {}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_visibility() -> CatalogVisibility {
        CatalogVisibility::new(true, true)
    }

    #[test]
    fn human_melee_browse_yields_categories_and_a_first_result() {
        let request = UnitListingRequest::new(
            Race::Human,
            UnitMode::Melee,
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        );
        let listing = UnitListing::resolve(&request);
        assert!(
            !listing.category_kinds().is_empty(),
            "a human melee browse should surface at least one category"
        );
        let first = listing
            .first_result()
            .expect("a human melee browse should have a first result");
        assert!(!first.unit_id().is_empty());
        assert!(
            listing.category_kinds().contains(&first.unit_kind()),
            "the first result's category must appear in the category list"
        );
    }

    #[test]
    fn category_kinds_are_deduped() {
        let request = UnitListingRequest::new(
            Race::Human,
            UnitMode::Melee,
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        );
        let listing = UnitListing::resolve(&request);
        let mut seen = listing.category_kinds().to_vec();
        seen.sort();
        let unique_len = {
            seen.dedup();
            seen.len()
        };
        assert_eq!(
            unique_len,
            listing.category_kinds().len(),
            "category kinds must contain no duplicates"
        );
    }

    #[test]
    fn category_browse_yields_named_entries() {
        let listing_request = UnitListingRequest::new(
            Race::Human,
            UnitMode::Melee,
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        );
        let listing = UnitListing::resolve(&listing_request);
        let category_kind = *listing
            .category_kinds()
            .first()
            .expect("human melee should have at least one category");
        let category_request = UnitCategoryRequest::new(
            Race::Human,
            UnitMode::Melee,
            category_kind,
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        );
        let category_listing = UnitCategoryListing::resolve(&category_request);
        assert!(
            !category_listing.entries().is_empty(),
            "the first category should contain at least one unit"
        );
        for entry in category_listing.entries() {
            assert!(!entry.unit_id().is_empty());
            assert!(!entry.display_name().is_empty());
            assert_eq!(entry.unit_kind(), category_kind);
        }
    }
}
