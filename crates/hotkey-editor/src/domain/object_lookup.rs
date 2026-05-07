use warcraft_api::WarcraftObject;
use warcraft_database::WARCRAFT_DATABASE;

pub(crate) struct ObjectLookup;

impl ObjectLookup {
    pub(crate) fn by_id(needle_id: &str) -> Option<&'static WarcraftObject> {
        WARCRAFT_DATABASE.by_id(needle_id)
    }

    pub(crate) fn has_icon(object_id: &str) -> bool {
        let database_object = WARCRAFT_DATABASE.by_id(object_id);
        database_object.is_some_and(|object| object.has_displayable_icon())
    }

    pub(crate) fn is_passive_ability(object_id: &str) -> bool {
        let database_object = WARCRAFT_DATABASE.by_id(object_id);
        database_object.is_some_and(|object| object.is_passive_ability())
    }

    pub(crate) fn morph_target_unit(object_id: &str) -> Option<&'static str> {
        let database_object = WARCRAFT_DATABASE.by_id(object_id);
        database_object.and_then(|object| object.ability_morph_target_id())
    }

    pub(crate) fn ability_code(object_id: &str) -> Option<&'static str> {
        let database_object = WARCRAFT_DATABASE.by_id(object_id);
        database_object.and_then(|object| object.ability_code())
    }

    pub(crate) fn off_icon(object_id: &str) -> Option<&'static str> {
        let database_object = WARCRAFT_DATABASE.by_id(object_id);
        database_object.and_then(|object| object.ability_off_icon())
    }
}
