use super::components::unit_name::UnitNameProps;
use super::props::UnitNameRowProps;

impl From<&UnitNameRowProps> for UnitNameProps {
    fn from(props: &UnitNameRowProps) -> Self {
        let text = props.unit_name;
        Self { text }
    }
}
