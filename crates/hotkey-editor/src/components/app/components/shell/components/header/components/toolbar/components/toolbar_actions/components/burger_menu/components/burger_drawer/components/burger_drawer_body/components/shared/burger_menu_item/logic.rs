use super::components::active_menu_item::ActiveMenuItemProps;
use super::components::idle_menu_item::IdleMenuItemProps;
use super::components::primary_menu_item::PrimaryMenuItemProps;
use super::props::BurgerMenuItemProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_icon::BurgerMenuItemIconProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_label::BurgerMenuItemLabelProps;

impl From<&BurgerMenuItemProps> for BurgerMenuItemIconProps {
    fn from(props: &BurgerMenuItemProps) -> Self {
        let svg = props.icon;
        Self { svg }
    }
}

impl From<&BurgerMenuItemProps> for BurgerMenuItemLabelProps {
    fn from(props: &BurgerMenuItemProps) -> Self {
        let text = props.label.clone();
        Self { text }
    }
}

impl From<&BurgerMenuItemProps> for IdleMenuItemProps {
    fn from(props: &BurgerMenuItemProps) -> Self {
        let icon = BurgerMenuItemIconProps::from(props);
        let label = BurgerMenuItemLabelProps::from(props);
        let disabled = props.disabled;
        let role = props.role;
        let aria_haspopup = props.aria_haspopup;
        let aria_expanded = props.aria_expanded;
        let aria_pressed = props.aria_pressed;
        let aria_label = props.aria_label;
        let onclick = props.onclick;
        Self {
            icon,
            label,
            disabled,
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            onclick,
        }
    }
}

impl From<&BurgerMenuItemProps> for ActiveMenuItemProps {
    fn from(props: &BurgerMenuItemProps) -> Self {
        let icon = BurgerMenuItemIconProps::from(props);
        let label = BurgerMenuItemLabelProps::from(props);
        let disabled = props.disabled;
        let role = props.role;
        let aria_haspopup = props.aria_haspopup;
        let aria_expanded = props.aria_expanded;
        let aria_pressed = props.aria_pressed;
        let aria_label = props.aria_label;
        let onclick = props.onclick;
        Self {
            icon,
            label,
            disabled,
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            onclick,
        }
    }
}

impl From<&BurgerMenuItemProps> for PrimaryMenuItemProps {
    fn from(props: &BurgerMenuItemProps) -> Self {
        let icon = BurgerMenuItemIconProps::from(props);
        let label = BurgerMenuItemLabelProps::from(props);
        let disabled = props.disabled;
        let role = props.role;
        let aria_haspopup = props.aria_haspopup;
        let aria_expanded = props.aria_expanded;
        let aria_pressed = props.aria_pressed;
        let aria_label = props.aria_label;
        let onclick = props.onclick;
        Self {
            icon,
            label,
            disabled,
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            onclick,
        }
    }
}
