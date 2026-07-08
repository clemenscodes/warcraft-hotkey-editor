use tw_macro::internal::{join_into, joined_len};
use tw_macro::tw;
use tw_macro::{ClassList, TailwindClass};
use warcraft_api::Race;

classes! {
    base: tw![
        "flex-1",
        "min-w-0",
        "min-h-11",
        "px-2",
        "bg-warcraft-bg-mid/55",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "text-warcraft-text-secondary",
        "text-sm",
        "tracking-label",
        "uppercase",
        "text-center",
        "cursor-pointer",
        "transition-all",
        "duration-fast",
        "whitespace-nowrap",
        "overflow-hidden",
        "text-ellipsis",
        "hover:bg-warcraft-blue-deep/70",
        "hover:text-white",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]", "kb-focus:shadow-focus",
        "data-[active=true]:bg-panel-blue",
    ],
    mobile: tw![
        "mobile:text-xs",
        "mobile:px-1.5",
        "mobile:h-11",
        "mobile:leading-none",
    ],
    tablet: tw![
        "tablet:text-xs",
        "tablet:px-1.5",
        "tablet:h-11",
        "tablet:leading-none",
    ],
}

// The tab wears its OWN race's accent — a per-tab data→token mapping keyed on the
// race, chosen directly from the race rather than a cascaded var. A plain `match`
// layering that race's hover/active accent overlay onto the shared base (via a small
// local join macro), never a `states!` table. The active/inactive distinction is a
// separate concern handled by the `data-[active]` variants inside each arm and the
// base, driven by the button's `data-active`.
macro_rules! mobile_category_tab_class {
    ($($utility:literal),+ $(,)?) => {{
        const OVERLAY: &[TailwindClass] = tw![$($utility),+];
        const LEN: usize = joined_len(CLASS_STR, &[OVERLAY]);
        const BYTES: [u8; LEN] = join_into::<LEN>(CLASS_STR, &[OVERLAY]);
        const JOINED: ClassList = ClassList::new(match ::core::str::from_utf8(&BYTES) {
            ::core::result::Result::Ok(class) => class,
            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 mobile category tab class"),
        });
        JOINED
    }};
}

pub(super) fn class(race: Race) -> ClassList {
    match race {
        Race::Human => mobile_category_tab_class![
            "hover:border-race-human",
            "data-[active=true]:border-race-human",
            "data-[active=true]:text-race-human",
            "data-[active=true]:[--glow-color:var(--color-race-human)]",
            "data-[active=true]:shadow-glow-soft",
        ],
        Race::Nightelf => mobile_category_tab_class![
            "hover:border-race-nightelf",
            "data-[active=true]:border-race-nightelf",
            "data-[active=true]:text-race-nightelf",
            "data-[active=true]:[--glow-color:var(--color-race-nightelf)]",
            "data-[active=true]:shadow-glow-soft",
        ],
        Race::Orc => mobile_category_tab_class![
            "hover:border-race-orc",
            "data-[active=true]:border-race-orc",
            "data-[active=true]:text-race-orc",
            "data-[active=true]:[--glow-color:var(--color-race-orc)]",
            "data-[active=true]:shadow-glow-soft",
        ],
        Race::Undead => mobile_category_tab_class![
            "hover:border-race-undead",
            "data-[active=true]:border-race-undead",
            "data-[active=true]:text-race-undead",
            "data-[active=true]:[--glow-color:var(--color-race-undead)]",
            "data-[active=true]:shadow-glow-soft",
        ],
        Race::Neutral => mobile_category_tab_class![
            "hover:border-warcraft-gold",
            "data-[active=true]:border-warcraft-gold",
            "data-[active=true]:text-warcraft-gold",
            "data-[active=true]:shadow-glow-soft",
        ],
    }
}
