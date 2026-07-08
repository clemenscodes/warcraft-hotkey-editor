use tw_macro::tw;
classes! {
    base: tw![
        "flex-[0_0_auto]",
        "text-warcraft-success",
        "text-xl",
        "font-normal",
        "[font-variant-numeric:tabular-nums]",
        "text-shadow-drop",
        "data-[zero=true]:text-warcraft-text-faint",
        "data-[zero=true]:font-normal",
    ],
    mobile: tw!["mobile:text-xl"],
}
