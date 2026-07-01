/// Which stat category a column shows, driving its named grid area.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum StatColumnKind {
    Vitality,
    Combat,
    Defense,
    Attributes,
}

impl StatColumnKind {
    pub(super) fn data_attr(self) -> &'static str {
        match self {
            Self::Vitality => "vitality",
            Self::Combat => "combat",
            Self::Defense => "defense",
            Self::Attributes => "attributes",
        }
    }
}
