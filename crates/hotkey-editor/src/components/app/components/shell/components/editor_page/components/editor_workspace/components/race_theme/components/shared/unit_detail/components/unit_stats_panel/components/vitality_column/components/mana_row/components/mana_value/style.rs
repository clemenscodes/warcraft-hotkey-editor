use tw_macro::tw;
classes! {
    base: tw![
        "flex-[1_1_auto]",
        "min-w-0",
        "text-right",
        "text-race-human",
        "font-semibold",
        "text-2xl",
        "[font-variant-numeric:tabular-nums]",
        "data-[zero=true]:text-warcraft-text-faint",
        "data-[zero=true]:font-normal",
    ],
    mobile: tw!["mobile:text-3xl"],
}
