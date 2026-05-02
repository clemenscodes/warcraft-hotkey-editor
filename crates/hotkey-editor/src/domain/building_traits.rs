use crate::domain::object_lookup::ObjectLookup;

pub(crate) struct BuildingTraits;

impl BuildingTraits {
    pub(crate) fn can_attack(object_id: &str) -> bool {
        matches!(
            object_id.to_ascii_lowercase().as_str(),
            "hgtw"
                | "hatw"
                | "hctw"
                | "owtw"
                | "otrb"
                | "unp1"
                | "unp2"
                | "uzg1"
                | "uzg2"
                | "nadt"
                | "ndgt"
                | "ntt1"
        )
    }

    pub(crate) fn can_uproot(object_id: &str) -> bool {
        matches!(
            object_id.to_ascii_lowercase().as_str(),
            "etol" | "etoa" | "etoe" | "eaow" | "eaoe" | "eaom" | "etrp" | "eden"
        )
    }

    pub(crate) fn unit_starts_in_toggle_alt_state(unit_id: &str) -> bool {
        if Self::can_uproot(unit_id) {
            return true;
        }
        if Self::is_burrowed_form(unit_id) {
            return true;
        }
        matches!(unit_id.to_ascii_lowercase().as_str(), "hmil")
    }

    pub(crate) fn is_burrowed_form(unit_id: &str) -> bool {
        let Some(warcraft_object) = ObjectLookup::by_id(unit_id) else {
            return false;
        };
        warcraft_object
            .names()
            .first()
            .copied()
            .map(|first_name| first_name.to_ascii_lowercase().starts_with("burrowed "))
            .unwrap_or(false)
    }

    pub(crate) fn ability_has_alt_state(ability_id: &str) -> bool {
        let Some(warcraft_object) = ObjectLookup::by_id(ability_id) else {
            return false;
        };
        warcraft_object.un_tip().is_some() || warcraft_object.un_ubertip().is_some()
    }
}
