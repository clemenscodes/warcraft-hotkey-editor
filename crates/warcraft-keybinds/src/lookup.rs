use std::borrow::Borrow;

use warcraft_api::{WarcraftObject, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

const ICON_PATH_BLACKLIST: &[&str] = &["commandbuttons/btnselectheroon.blp"];

fn is_known_bad_icon(icon_path: &str) -> bool {
    let normalized = icon_path.trim().to_ascii_lowercase();
    ICON_PATH_BLACKLIST
        .iter()
        .any(|blacklisted| *blacklisted == normalized)
}

pub struct ObjectLookup;

impl ObjectLookup {
    pub fn by_id(needle_id: &str) -> Option<&'static WarcraftObject> {
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

    pub fn has_icon(object_id: &str) -> bool {
        let Some(warcraft_object) = Self::by_id(object_id) else {
            return false;
        };
        warcraft_object
            .icons()
            .iter()
            .any(|icon_path| !icon_path.trim().is_empty() && !is_known_bad_icon(icon_path))
    }

    pub fn is_passive_ability(object_id: &str) -> bool {
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

    pub fn ability_code(object_id: &str) -> Option<&'static str> {
        let warcraft_object = Self::by_id(object_id)?;
        match warcraft_object.meta() {
            WarcraftObjectMeta::Ability(meta) => meta.code(),
            _ => None,
        }
    }

    pub fn morph_target_unit(object_id: &str) -> Option<&'static str> {
        let warcraft_object = Self::by_id(object_id)?;
        match warcraft_object.meta() {
            WarcraftObjectMeta::Ability(meta) => meta.morph_target_unit().map(|id| id.value()),
            _ => None,
        }
    }

    pub fn off_icon(object_id: &str) -> Option<&'static str> {
        let warcraft_object = Self::by_id(object_id)?;
        match warcraft_object.meta() {
            WarcraftObjectMeta::Ability(meta) => meta.off_icon(),
            _ => None,
        }
    }
}
