use tw_macro::internal::{join_into, joined_len};
use tw_macro::tw;
use tw_macro::{ClassList, TailwindClass};
use warcraft_api::Race;

// Phone/tablet: a swipe-scannable banner-card strip — each tab a chunky fixed
// height (all five share the row) with tighter type and a slightly softer banner
// brightness than the desktop row.

classes! {
    base: tw![
        "group",
        "relative",
        "flex-1",
        "min-w-0",
        "p-0",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "text-warcraft-text-primary",
        "text-xl",
        "uppercase",
        "tracking-caps",
        "text-center",
        "transition-[border-color,box-shadow,transform]",
        "duration-base",
        "overflow-hidden",
        "isolate",
        "flex",
        "items-end",
        "justify-center",
        "h-full",
        "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),-1px_1px_0_var(--color-warcraft-shadow),1px_-1px_0_var(--color-warcraft-shadow),-1px_-1px_0_var(--color-warcraft-shadow),0_0_8px_color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)]",
        "before:content-['']",
        "before:absolute",
        "before:inset-0",
        "before:bg-contain",
        "before:bg-no-repeat",
        "before:bg-center",
        "before:brightness-150",
        "before:saturate-125",
        "before:z-0",
        "before:pointer-events-none",
        "before:block",
        "after:content-['']",
        "after:absolute",
        "after:inset-0",
        "after:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-shadow)_0%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_0%,transparent)_45%,color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)_75%,color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)_100%)]",
        "after:z-1",
        "after:pointer-events-none",
        "after:block",
        "hover:text-white",
        "focus:outline-none",
        "data-[active=true]:text-white",
        "kb-focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:border-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]", "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:before:brightness-[1.35]",
        "mobile:before:saturate-[1.2]",
    ],
    tablet: tw![
        "tablet:text-sm",
        "tablet:before:brightness-[1.35]",
        "tablet:before:saturate-[1.2]",
    ],
}

// Each race tab wears its OWN race's banner and accent — a per-tab data→token
// mapping, not active-race theming: the banner art, the `--race-color`, and the
// hover/active border-and-glow all follow the race the tab represents. This is a
// plain `match` layering that race's overlay onto the shared base (via a small local
// join macro), never a `states!` table: the tab is not one element with N
// mutually-exclusive runtime states, it is one look parameterised by its race. The
// active/inactive distinction is a separate concern handled by the `data-[active]`
// variants inside each arm and the base, driven by the button's `data-active`.
macro_rules! race_tab_class {
    ($($utility:literal),+ $(,)?) => {{
        const OVERLAY: &[TailwindClass] = tw![$($utility),+];
        const LEN: usize = joined_len(CLASS_STR, &[OVERLAY]);
        const BYTES: [u8; LEN] = join_into::<LEN>(CLASS_STR, &[OVERLAY]);
        const JOINED: ClassList = ClassList::new(match ::core::str::from_utf8(&BYTES) {
            ::core::result::Result::Ok(class) => class,
            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 race tab class"),
        });
        JOINED
    }};
}

pub(super) fn class(race: Race) -> ClassList {
    match race {
        Race::Human => race_tab_class![
            "[--race-color:var(--color-race-human)]",
            "bg-race-banner-soft",
            "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-human.png')]",
            "hover:border-race-human",
            "hover:[--glow-color:var(--color-race-human)]",
            "hover:shadow-glow",
            "data-[active=true]:border-race-human",
            "data-[active=true]:[--glow-color:var(--color-race-human)]",
            "data-[active=true]:shadow-glow-strong",
        ],
        Race::Orc => race_tab_class![
            "[--race-color:var(--color-race-orc-strong)]",
            "bg-race-banner-strong",
            "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-orc.png')]",
            "hover:border-race-orc",
            "hover:[--glow-color:var(--color-race-orc)]",
            "hover:shadow-glow",
            "data-[active=true]:border-race-orc",
            "data-[active=true]:[--glow-color:var(--color-race-orc)]",
            "data-[active=true]:shadow-glow-strong",
        ],
        Race::Nightelf => race_tab_class![
            "[--race-color:var(--color-race-nightelf)]",
            "bg-race-banner-soft",
            "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-nightelf.png')]",
            "hover:border-race-nightelf",
            "hover:[--glow-color:var(--color-race-nightelf)]",
            "hover:shadow-glow",
            "data-[active=true]:border-race-nightelf",
            "data-[active=true]:[--glow-color:var(--color-race-nightelf)]",
            "data-[active=true]:shadow-glow-strong",
        ],
        Race::Undead => race_tab_class![
            "[--race-color:var(--color-race-undead)]",
            "bg-race-banner-soft",
            "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-undead.png')]",
            "hover:border-race-undead",
            "hover:[--glow-color:var(--color-race-undead)]",
            "hover:shadow-glow",
            "data-[active=true]:border-race-undead",
            "data-[active=true]:[--glow-color:var(--color-race-undead)]",
            "data-[active=true]:shadow-glow-strong",
        ],
        Race::Neutral => race_tab_class![
            "[--race-color:var(--color-race-neutral-strong)]",
            "bg-race-banner-strong",
            "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-random.png')]",
            "hover:border-warcraft-gold",
            "hover:shadow-glow",
            "data-[active=true]:border-warcraft-gold",
            "data-[active=true]:shadow-glow-strong",
        ],
    }
}
