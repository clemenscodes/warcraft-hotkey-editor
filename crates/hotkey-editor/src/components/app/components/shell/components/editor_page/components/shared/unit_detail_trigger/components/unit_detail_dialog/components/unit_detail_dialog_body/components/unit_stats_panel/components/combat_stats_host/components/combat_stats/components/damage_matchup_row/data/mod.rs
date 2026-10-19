use warcraft_api::DefenseType;

pub(super) const DISPLAYED_DEFENSE_TYPES: [DefenseType; 7] = [
    DefenseType::Light,
    DefenseType::Medium,
    DefenseType::Heavy,
    DefenseType::Fortified,
    DefenseType::Hero,
    DefenseType::Divine,
    DefenseType::Unarmored,
];
