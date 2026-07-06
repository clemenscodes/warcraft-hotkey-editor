use tw_macro::tw;
classes! {
    base: tw![
        "uppercase",
        "tracking-[0.04em]",
        "text-[3.4rem]",
        "leading-none",
        "whitespace-nowrap",
        "text-warcraft-gold",
        "text-shadow-glow-14",
        "data-[compact=true]:text-[2.4rem]",
        "data-[conflict=true]:text-warcraft-danger",
        "data-[conflict=true]:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_14px_color-mix(in_oklab,var(--color-warcraft-danger)_55%,transparent)]",
    ],
    mobile: tw![
        "mobile:text-[clamp(11px,3.6vw,15px)]",
        "mobile:tracking-[0.02em]",
        "mobile:data-[compact=true]:text-[clamp(14px,4vw,18px)]",
    ],
    tablet: tw![
        "tablet:text-[clamp(11px,3.6vw,15px)]",
        "tablet:tracking-[0.02em]",
        "tablet:data-[compact=true]:text-[clamp(14px,4vw,18px)]",
    ],
}
