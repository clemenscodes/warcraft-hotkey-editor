use super::model::UnitDetailTriggerModel;
use crate::components::app::components::shell::components::shared::dialog_toggle::use_dialog_toggle;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

pub(super) struct UnitDetailTriggerPresentation {
    pub(super) icon_url: Option<String>,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) open: bool,
    pub(super) onpointerdown: EventHandler<PointerEvent>,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

impl ddd::Layered for UnitDetailTriggerPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for UnitDetailTriggerPresentation {
    type Model = UnitDetailTriggerModel;
}

pub(super) fn use_unit_detail_trigger(
    props: &UnitDetailTriggerModel,
) -> UnitDetailTriggerPresentation {
    let icon_url = props.icon_url.clone();
    let unit_id = props.unit_id;
    let open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    // A press on the button while the dialog is open dismisses it before the
    // click arrives, so the toggle decides from what the dialog was at the press.
    let toggle = use_dialog_toggle(open_signal);
    let onpointerdown = toggle.onpointerdown();
    let onclick = toggle.onclick();
    let on_open_change = toggle.on_open_change();
    UnitDetailTriggerPresentation {
        icon_url,
        unit_id,
        open,
        onpointerdown,
        onclick,
        on_open_change,
    }
}
