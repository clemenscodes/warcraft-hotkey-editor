use tw_macro::tw;
classes! {
    base: tw![
        "uppercase",
        "tracking-label",
        "text-4xl",
        "leading-none",
        "whitespace-nowrap",
        "text-warcraft-gold",
        "text-shadow-glow",
        "data-[compact=true]:text-3xl",
        "data-[conflict=true]:text-warcraft-danger",
        "data-[conflict=true]:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_14px_color-mix(in_oklab,var(--color-warcraft-danger)_55%,transparent)]",
    ],
    mobile: tw![
        "mobile:text-xs",
        "mobile:tracking-snug",
        "mobile:data-[compact=true]:text-base",
    ],
    tablet: tw![
        "tablet:text-xs",
        "tablet:tracking-snug",
        "tablet:data-[compact=true]:text-base",
    ],
}
