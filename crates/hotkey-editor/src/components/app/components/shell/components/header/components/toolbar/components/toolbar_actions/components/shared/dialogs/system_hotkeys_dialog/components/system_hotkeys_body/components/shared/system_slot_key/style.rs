use tw_macro::tw;
classes! {
    base: tw![
        "uppercase",
        "tracking-[0.04em]",
        "text-4xl",
        "leading-none",
        "whitespace-nowrap",
        "text-warcraft-gold",
        "text-shadow-glow-14",
        "data-[compact=true]:text-3xl",
        "data-[conflict=true]:text-warcraft-danger",
        "data-[conflict=true]:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_14px_color-mix(in_oklab,var(--color-warcraft-danger)_55%,transparent)]",
    ],
    mobile: tw![
        "mobile:text-xs",
        "mobile:tracking-[0.02em]",
        "mobile:data-[compact=true]:text-base",
    ],
    tablet: tw![
        "tablet:text-xs",
        "tablet:tracking-[0.02em]",
        "tablet:data-[compact=true]:text-base",
    ],
}
