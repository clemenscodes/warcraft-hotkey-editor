use warcraft_api::Race;

pub(super) struct UnitCardClasses {
    button_class: &'static str,
    id_class: &'static str,
}

impl UnitCardClasses {
    pub(super) fn compute(is_selected: bool, race: Race) -> Self {
        let button_class = if is_selected {
            Self::selected_button_class(race)
        } else {
            "unit-card"
        };
        let id_class = if is_selected {
            Self::selected_id_class(race)
        } else {
            "unit-card-id"
        };
        Self {
            button_class,
            id_class,
        }
    }

    fn selected_button_class(race: Race) -> &'static str {
        match race {
            Race::Human => "unit-card selected race-human",
            Race::Orc => "unit-card selected race-orc",
            Race::Nightelf => "unit-card selected race-nightelf",
            Race::Undead => "unit-card selected race-undead",
            Race::Neutral => "unit-card selected race-neutral",
        }
    }

    fn selected_id_class(race: Race) -> &'static str {
        match race {
            Race::Human => "unit-card-id race-human",
            Race::Orc => "unit-card-id race-orc",
            Race::Nightelf => "unit-card-id race-nightelf",
            Race::Undead => "unit-card-id race-undead",
            Race::Neutral => "unit-card-id race-neutral",
        }
    }

    pub(super) fn button_class(&self) -> &'static str {
        self.button_class
    }

    pub(super) fn id_class(&self) -> &'static str {
        self.id_class
    }
}
