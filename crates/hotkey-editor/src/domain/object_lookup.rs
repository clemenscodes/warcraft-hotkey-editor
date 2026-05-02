use std::borrow::Borrow;

use warcraft_api::WarcraftObject;
use warcraft_database::WARCRAFT_DATABASE;

pub(crate) struct ObjectLookup;

impl ObjectLookup {
    pub(crate) fn by_id(needle_id: &str) -> Option<&'static WarcraftObject> {
        let database_map = WARCRAFT_DATABASE.db();
        let direct_lookup: Option<&WarcraftObject> = database_map.get(needle_id);
        if direct_lookup.is_some() {
            return direct_lookup;
        }
        for (object_id, warcraft_object) in database_map.iter() {
            let id_value: &str = object_id.borrow();
            if id_value.eq_ignore_ascii_case(needle_id) {
                return Some(warcraft_object);
            }
        }
        None
    }

    pub(crate) fn has_icon(object_id: &str) -> bool {
        let Some(warcraft_object) = Self::by_id(object_id) else {
            return false;
        };
        warcraft_object
            .icons()
            .iter()
            .any(|icon_path| !icon_path.trim().is_empty())
    }

    pub(crate) fn is_passive_ability(object_id: &str) -> bool {
        let Some(warcraft_object) = Self::by_id(object_id) else {
            return false;
        };
        warcraft_object
            .icons()
            .first()
            .copied()
            .map(|icon_path| {
                icon_path
                    .to_ascii_lowercase()
                    .starts_with("passivebuttons/")
            })
            .unwrap_or(false)
    }
}
