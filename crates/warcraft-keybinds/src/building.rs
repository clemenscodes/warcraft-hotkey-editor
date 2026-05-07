use warcraft_api::WarcraftObjectMeta;
use warcraft_database::WARCRAFT_DATABASE;

pub struct BuildingTraits;

impl BuildingTraits {
    pub fn can_attack(object_id: &str) -> bool {
        let lowercase_id = object_id.to_ascii_lowercase();
        matches!(
            lowercase_id.as_str(),
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

    pub fn can_uproot(object_id: &str) -> bool {
        let lowercase_id = object_id.to_ascii_lowercase();
        matches!(
            lowercase_id.as_str(),
            "etol" | "etoa" | "etoe" | "eaow" | "eaoe" | "eaom" | "etrp" | "eden"
        )
    }

    pub fn unit_starts_in_toggle_alt_state(unit_id: &str) -> bool {
        if Self::can_uproot(unit_id) {
            return true;
        }
        if Self::is_burrowed_form(unit_id) {
            return true;
        }
        unit_id.eq_ignore_ascii_case("hmil")
    }

    pub fn ability_is_on_alt_state_unit(ability_id: &str) -> bool {
        for (unit_id_obj, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let unit_id = unit_id_obj.value();
            if !Self::unit_starts_in_toggle_alt_state(unit_id) {
                continue;
            }
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            let has_ability = unit_meta.abilities().iter().any(|ability_id_obj| {
                let ability_object_id = ability_id_obj.value();
                ability_object_id.eq_ignore_ascii_case(ability_id)
            });
            if has_ability {
                return true;
            }
        }
        false
    }

    pub fn is_burrowed_form(unit_id: &str) -> bool {
        let Some(warcraft_object) = WARCRAFT_DATABASE.by_id(unit_id) else {
            return false;
        };
        let names = warcraft_object.names();
        let Some(first_name) = names.first().copied() else {
            return false;
        };
        let lowercase_name = first_name.to_ascii_lowercase();
        lowercase_name.starts_with("burrowed ")
    }

    pub fn ability_has_alt_state(ability_id: &str) -> bool {
        let Some(warcraft_object) = WARCRAFT_DATABASE.by_id(ability_id) else {
            return false;
        };
        warcraft_object.un_tip().is_some() || warcraft_object.un_ubertip().is_some()
    }
}
