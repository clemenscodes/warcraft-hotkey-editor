use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "gap-2",
        "text-xl",
        "leading-title",
        "min-w-0",
        "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
        "[--attribute-label-color:var(--color-warcraft-gold)]",
    ],
    mobile: tw![
        "mobile:text-2xl",
        "mobile:leading-heading",
    ],
}
