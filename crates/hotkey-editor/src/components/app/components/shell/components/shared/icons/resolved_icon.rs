use super::IconUrl;
use warcraft_api::{WarcraftApi, WarcraftObjectId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResolvedIcon {
    name: Option<String>,
    icon_url: Option<String>,
}

impl ResolvedIcon {
    pub fn lookup(object_id: WarcraftObjectId) -> Self {
        let api = WarcraftApi::default();
        let object_option = api.object(object_id);
        let icon_url = object_option
            .and_then(|object| object.icons().first().copied())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        let name = object_option
            .and_then(|object| object.names().first().copied())
            .map(str::to_owned);
        Self { name, icon_url }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn name_or(&self, fallback_id: WarcraftObjectId) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => fallback_id.value().to_owned(),
        }
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
}
