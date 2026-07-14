use super::model::AbilityIconModel;
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

pub(super) struct AbilityIconPresentation {
    pub(super) open_state: Signal<Option<InspectedAbility>>,
    pub(super) icon_src: Option<String>,
    pub(super) icon_alt: String,
    pub(super) count: usize,
    pub(super) is_winner: bool,
    pub(super) title: String,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_ability_icon(props: &AbilityIconModel) -> AbilityIconPresentation {
    let mut open_state = use_signal(|| None::<InspectedAbility>);
    let inspected = props.inspected.clone();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let opened = inspected.clone();
        open_state.set(Some(opened));
    });
    let name = props.name.clone();
    let carrier_count = props.carrier_count;
    let title = format!("{name} — {carrier_count} carriers");
    let icon_src = props.icon_url.clone();
    let icon_alt = name;
    let count = carrier_count;
    let is_winner = props.is_winner;
    let disabled = props.disabled;
    AbilityIconPresentation {
        open_state,
        icon_src,
        icon_alt,
        count,
        is_winner,
        title,
        disabled,
        onclick,
    }
}

impl ddd::Presentation for AbilityIconPresentation {
    type Model = AbilityIconModel;
}
