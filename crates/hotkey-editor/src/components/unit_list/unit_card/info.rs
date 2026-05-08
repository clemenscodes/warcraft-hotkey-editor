use dioxus::prelude::*;

#[component]
pub(super) fn UnitCardInfo(
    display_name: String,
    unit_id: String,
    id_class: &'static str,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-[0.45rem] min-w-0 flex-1",
            span {
                class: "text-[1.05rem] leading-[1.25] pb-[0.1rem] overflow-hidden \
                        text-ellipsis whitespace-nowrap min-w-0 min-[1900px]:text-[1.35rem]",
                "{display_name}"
            }
            code { class: id_class, "{unit_id}" }
        }
    }
}
