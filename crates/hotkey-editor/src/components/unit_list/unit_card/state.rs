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
            "unit-card flex items-center gap-4 p-4 bg-[rgba(13,31,61,0.55)] border border-[#1f3d63] rounded-[6px] text-left text-warcraft-text-primary font-friz-quadrata text-[1.4rem] tracking-[0.02em] transition-all duration-[120ms] min-w-0 w-full hover:bg-[rgba(30,60,95,0.7)] hover:text-white focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:border-white [body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:bg-[rgba(40,80,130,0.85)]"
        };
        let id_class = if is_selected {
            Self::selected_id_class(race)
        } else {
            "font-mono text-[1.05rem] leading-[1.2] text-[#7b818d] overflow-hidden text-ellipsis whitespace-nowrap"
        };
        Self {
            button_class,
            id_class,
        }
    }

    fn selected_button_class(race: Race) -> &'static str {
        match race {
            Race::Human => {
                "unit-card flex items-center gap-4 p-4 bg-gradient-to-br from-[rgba(45,80,130,0.9)] to-[rgba(20,45,80,0.9)] border border-race-human rounded-[6px] text-left text-race-human font-friz-quadrata text-[1.4rem] tracking-[0.02em] transition-all duration-[120ms] min-w-0 w-full shadow-[0_0_8px_rgba(106,161,255,0.3)] hover:text-white focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:border-white [body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:bg-[rgba(40,80,130,0.85)]"
            }
            Race::Orc => {
                "unit-card flex items-center gap-4 p-4 bg-gradient-to-br from-[rgba(45,80,130,0.9)] to-[rgba(20,45,80,0.9)] border border-race-orc rounded-[6px] text-left text-race-orc font-friz-quadrata text-[1.4rem] tracking-[0.02em] transition-all duration-[120ms] min-w-0 w-full shadow-[0_0_8px_rgba(255,122,122,0.3)] hover:text-white focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:border-white [body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:bg-[rgba(40,80,130,0.85)]"
            }
            Race::Nightelf => {
                "unit-card flex items-center gap-4 p-4 bg-gradient-to-br from-[rgba(45,80,130,0.9)] to-[rgba(20,45,80,0.9)] border border-race-nightelf rounded-[6px] text-left text-race-nightelf font-friz-quadrata text-[1.4rem] tracking-[0.02em] transition-all duration-[120ms] min-w-0 w-full shadow-[0_0_8px_rgba(95,218,218,0.3)] hover:text-white focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:border-white [body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:bg-[rgba(40,80,130,0.85)]"
            }
            Race::Undead => {
                "unit-card flex items-center gap-4 p-4 bg-gradient-to-br from-[rgba(45,80,130,0.9)] to-[rgba(20,45,80,0.9)] border border-race-undead rounded-[6px] text-left text-race-undead font-friz-quadrata text-[1.4rem] tracking-[0.02em] transition-all duration-[120ms] min-w-0 w-full shadow-[0_0_8px_rgba(199,155,255,0.3)] hover:text-white focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:border-white [body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:bg-[rgba(40,80,130,0.85)]"
            }
            Race::Neutral => {
                "unit-card flex items-center gap-4 p-4 bg-gradient-to-br from-[rgba(45,80,130,0.9)] to-[rgba(20,45,80,0.9)] border border-warcraft-gold rounded-[6px] text-left text-warcraft-gold font-friz-quadrata text-[1.4rem] tracking-[0.02em] transition-all duration-[120ms] min-w-0 w-full shadow-[0_0_8px_rgba(255,206,99,0.3)] hover:text-white focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:border-white [body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:bg-[rgba(40,80,130,0.85)]"
            }
        }
    }

    fn selected_id_class(race: Race) -> &'static str {
        match race {
            Race::Human => {
                "font-mono text-[1.05rem] leading-[1.2] text-race-human opacity-70 overflow-hidden text-ellipsis whitespace-nowrap"
            }
            Race::Orc => {
                "font-mono text-[1.05rem] leading-[1.2] text-race-orc opacity-70 overflow-hidden text-ellipsis whitespace-nowrap"
            }
            Race::Nightelf => {
                "font-mono text-[1.05rem] leading-[1.2] text-race-nightelf opacity-70 overflow-hidden text-ellipsis whitespace-nowrap"
            }
            Race::Undead => {
                "font-mono text-[1.05rem] leading-[1.2] text-race-undead opacity-70 overflow-hidden text-ellipsis whitespace-nowrap"
            }
            Race::Neutral => {
                "font-mono text-[1.05rem] leading-[1.2] text-warcraft-gold opacity-70 overflow-hidden text-ellipsis whitespace-nowrap"
            }
        }
    }

    pub(super) fn button_class(&self) -> &'static str {
        self.button_class
    }

    pub(super) fn id_class(&self) -> &'static str {
        self.id_class
    }
}
