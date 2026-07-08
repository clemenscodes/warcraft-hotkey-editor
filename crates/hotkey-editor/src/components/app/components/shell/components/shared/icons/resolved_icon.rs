use super::IconUrl;
use warcraft_database::ObjectLookup;

/// A database object resolved for display: its first icon URL and first name, if
/// any. The single home for the `ObjectLookup → icon → name` resolution that the
/// unit and ability card views all repeat; each caller supplies its own name
/// fallback (a unit falls back to its id, an ability to its slot's display name).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResolvedIcon {
    name: Option<String>,
    icon_url: Option<String>,
}

impl ResolvedIcon {
    /// Resolve a database object id to its first icon URL and first display name.
    pub fn lookup(object_id: &str) -> Self {
        let object_option = ObjectLookup::by_id(object_id);
        let icon_url = object_option
            .and_then(|object| object.icons().first().copied())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        let name = object_option
            .and_then(|object| object.names().first().copied())
            .map(str::to_owned);
        Self { name, icon_url }
    }

    /// The resolved display name, if the object has one.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// The resolved display name, or `fallback` when the object has none.
    pub fn name_or(&self, fallback: &str) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => fallback.to_owned(),
        }
    }

    /// The resolved icon URL, if the object has one.
    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
}
