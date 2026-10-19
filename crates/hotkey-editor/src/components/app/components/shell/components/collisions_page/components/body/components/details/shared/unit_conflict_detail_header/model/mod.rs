use super::view::UnitConflictDetailHeaderView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitConflictDetailHeaderModel {
    pub unit: UnitIconView,
    pub count: usize,
}

impl From<&UnitConflictDetailHeaderView> for UnitConflictDetailHeaderModel {
    fn from(view: &UnitConflictDetailHeaderView) -> Self {
        let UnitConflictDetailHeaderView { unit, count } = view.clone();
        Self { unit, count }
    }
}

impl ddd::Layered for UnitConflictDetailHeaderModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnitConflictDetailHeaderModel {
    type View = UnitConflictDetailHeaderView;
}
