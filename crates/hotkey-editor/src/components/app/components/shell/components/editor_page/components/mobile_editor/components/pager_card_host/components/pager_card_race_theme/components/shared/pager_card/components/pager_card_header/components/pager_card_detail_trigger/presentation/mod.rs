use super::model::PagerCardDetailTriggerModel;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

pub(super) struct PagerCardDetailTriggerPresentation {
    pub(super) icon_url: Option<String>,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

impl ddd::Presentation for PagerCardDetailTriggerPresentation {
    type Model = PagerCardDetailTriggerModel;
}

pub(super) fn use_pager_card_detail_trigger(
    props: &PagerCardDetailTriggerModel,
) -> PagerCardDetailTriggerPresentation {
    let icon_url = props.icon_url.clone();
    let unit_id = props.unit_id;
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    PagerCardDetailTriggerPresentation {
        icon_url,
        unit_id,
        open,
        onclick,
        on_open_change,
    }
}
