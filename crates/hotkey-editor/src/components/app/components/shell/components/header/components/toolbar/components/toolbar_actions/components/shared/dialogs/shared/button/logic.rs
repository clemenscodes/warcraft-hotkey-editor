use super::components::primary_button::PrimaryButtonProps;
use super::components::secondary_button::SecondaryButtonProps;
use super::props::ButtonProps;

impl From<&ButtonProps> for PrimaryButtonProps {
    fn from(props: &ButtonProps) -> Self {
        let onclick = props.onclick;
        let label = props.label.clone();
        Self { onclick, label }
    }
}

impl From<&ButtonProps> for SecondaryButtonProps {
    fn from(props: &ButtonProps) -> Self {
        let onclick = props.onclick;
        let label = props.label.clone();
        Self { onclick, label }
    }
}
