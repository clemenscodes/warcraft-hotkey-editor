use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "items-baseline",
        "gap-[0.75em]",
        "leading-heading",
        "text-shadow-drop",
        "min-w-0",
        "[--attribute-label-color:color-mix(in_oklab,var(--color-warcraft-gold)_90%,transparent)]",
    ],
    mobile: tw![
        "mobile:gap-[2.09cqi]",
    ],
}
