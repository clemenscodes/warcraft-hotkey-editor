use tw_macro::tw;
classes! {
    base: tw![
        "group",
        "flex",
        "items-baseline",
        "gap-2",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
        "min-w-0",
        "data-[regen=true]:mt-[-0.2rem]",
        "data-[regen=true]:pl-5",
        "data-[primary=true]:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
    ],
    mobile: tw!["mobile:text-2xl", "mobile:leading-heading"],
}
