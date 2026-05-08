use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitMode {
    Melee,
    Campaign,
}

impl TryFrom<&str> for UnitMode {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "melee" => Ok(Self::Melee),
            "campaign" => Ok(Self::Campaign),
            _ => Err(()),
        }
    }
}

impl fmt::Display for UnitMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Melee => formatter.write_str("melee"),
            Self::Campaign => formatter.write_str("campaign"),
        }
    }
}
