use tw_macro::tw;
classes! {
    base: tw![
        "group",
        "flex",
        "items-baseline",
        "gap-[0.55rem]",
        "text-[clamp(1.35rem,0.95rem+0.32vw,1.7rem)]/[1.2]",
        "text-shadow-drop",
        "min-w-0",
        "data-[regen=true]:mt-[-0.2rem]",
        "data-[regen=true]:pl-[1.25rem]",
        "data-[primary=true]:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
    ],
    mobile: tw!["mobile:text-2xl", "mobile:leading-heading"],
}
