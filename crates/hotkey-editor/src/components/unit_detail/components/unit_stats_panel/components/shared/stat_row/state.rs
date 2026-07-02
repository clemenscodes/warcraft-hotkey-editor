/// A stat row's colour/size variant, driving the hit-points and mana treatments.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum StatRowVariant {
    #[default]
    Default,
    Hp,
    Mana,
}

impl StatRowVariant {
    pub(super) fn data_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Hp => "hp",
            Self::Mana => "mana",
        }
    }
}
